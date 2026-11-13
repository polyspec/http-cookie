<!-- doc-id: agents -->
<!-- source-sha256: 011fa9b9dc232b10c532f4b16a146298c54737ec856d990705d9ca3c4def68b7 -->
# AGENTS.md

http-cookie의 모든 변경에 적용하는 규칙이다. 라이브러리의 규칙은 [docs/spec/cookies.ko.md](docs/spec/cookies.ko.md)에 있다.

## 배치

- 루트에 Cargo workspace 하나(`Cargo.toml`, `Cargo.lock`)가 있고, 코어와 다섯 target의 바인딩 crate를 담는다. lint는
  루트 `Cargo.toml`과 `clippy.toml`의 것이다.
- `packages/rust/`: 모든 규칙을 담는 crate `polyspec-http-cookie`(CK-1.4). 이 crate는 생성기 없이 빌드되고
  시험된다. `packages/rust/interface.json`: 바인딩의 인터페이스 계약(CK-1.5). crate 패키지가 이 파일을
  담으므로(`packages/rust/Cargo.toml`의 `include`), 이 crate에 의존하는 레포는 Cargo를 통해 이 파일을 읽는다.
- `packages/python/`, `packages/ts/`, `packages/ts/wasm/`, `packages/php-ext/`, `packages/go/`: Python, Node,
  브라우저(WASM), PHP, Go 바인딩이며, 모든 계약이 적는 다섯 target이다(binding BD-27). Go의 C ABI는 crate
  `packages/go/internal/ffi`다. 각각은 `binding scaffold`가
  쓰는 얇은 패키지이고 추적된다: `Cargo.toml`, `generated/lib.rs`를 include하는 `src/lib.rs`,
  Node addon의 `build.rs`, 그 언어의 manifest(`pyproject.toml`, `package.json`,
  `composer.json`, `go.mod`). 이 파일들은 손으로 고치지 않는다. binding이 쓰는 것과 다르면
  `make bindings-check`가 실패한다. 각 바인딩의 `generated/`(`packages/go/internal/ffi/generated`와
  `packages/ts/wasm/generated` 포함)는 `make bindings`가 쓰고 추적되지 않는다. Go 모듈에서 binding은 디렉터리 대신
  `zz_generated_*.go` 파일을 소유하고, 이 파일들은 추적된다(binding GN-3). `go get`은 모듈을 추적된 파일에서 받기
  때문이다. `make bindings`가 이 파일들을 다시 쓴다. 각 바인딩의 `tests/`는 그 언어로 contract case를 돌리는 runner를 담는다(CK-14.5).
- `contracts/cookies.json`: 규칙의 case(CK-14).
- `scripts/binding.mts`: binding을 실행한다("바인딩" 절을 본다).
- `docs/spec/`: 명세. 루트: `AGENTS.md`, `README.md`, `CHANGELOG.md`와 그 한국어 번역
  (`*.ko.md`), `Makefile`, `rust-toolchain.toml`, `clippy.toml`, `rustfmt.toml`, 그리고
  개발 도구의 `package.json`, `package-lock.json`, `composer.json`, `composer.lock`, 그리고 checkout의 도구의
  `mise.toml`, `mise.lock`, `config/toolchain.json`, `.php-version`, `.npmrc`, `.mise/locks/`("바인딩" 절을 본다).
- 바인딩 옆에 지우거나 남겨 둘 다른 언어의 구현은 없다.

## 바인딩

- 이 저장소는 하나의 Cargo workspace다. 코어 `packages/rust`와 다섯 target(binding BD-27)의 바인딩 crate `packages/python`,
  `packages/ts`, `packages/ts/wasm`, `packages/php-ext`, `packages/go/internal/ffi`(와 `packages/go/go.mod`)로 이루어지며, 바인딩 crate는 `binding scaffold`가 쓰고 `make bindings-check`가 비교한다.
  바인딩 crate의 `generated/`는 추적하지 않는다. 루트의 `Cargo.lock`이 모든 의존성을 고정한다.
- `polyspec-binding-core`는 binding이 릴리스될 때까지 코어와 바인딩 crate의 Git 의존성이다(binding 저장소의
  `v1/rust-core` 브랜치). `Cargo.lock`이 그 커밋을 고정한다. 코어는 자기 오류에
  `binding_core::Kinds`를 구현한다(CK-13.2, CK-13.3).
- binding 자체는 같은 저장소의 TypeScript 도구이며, 아직 어떤 registry에도 없다. `scripts/binding.mts`는
  `Cargo.lock`이 고정한 커밋으로 Cargo가 만든 checkout에서 binding을 가져와 `var/binding/<commit>`에 clone하고,
  거기에 binding의 `package-lock.json`대로 의존성을 설치한다. 그래서 생성기와 `binding_core`가 한 판이 된다.
  그 clone은 그 커밋의 binding Git checkout이다. binding check는 그 clone에서 자기 커밋을 읽어
  `Cargo.lock`의 커밋과 비교하며(CK-20), 파일만 복사한 것은 이 비교에서 실패한다. 이 방식은 binding의 첫 릴리스까지만
  유지된다.
- `contracts/cookies.json`의 case는 binding의 case 규약(G-5)을 따른다. case의 key는 snake_case로 쓴 parameter
  이름(`lines`, `name`, `value`, `attributes`)이고, record는 그 field들의 객체다(attributes는
  `{"path", "max_age", "secure", "http_only", "same_site"}`다).
- `make bindings`가 생성한다. 바인딩을 빌드하거나 시험하는 모든 make target은 그것을 먼저 실행한다. `make test-bindings`는
  workspace 전체에 clippy를 실행하고, `make check-declarations`를 실행하고, 그다음 계약의 다섯 target마다 하나씩
  `make test-python`, `make test-node`, `make test-wasm`, `make test-php`, `make test-go`를 실행한다. 각각은 binding이
  생성한 시험과 그 바인딩의 contract case를 그 언어의 표준 runner로 실행한다. runner는 unittest, node --test(Node와
  WASM, WASM 바인딩은 같은 설치 패키지의 subpath `./wasm`으로), PHPUnit, go test다. contract case는
  `packages/python/tests`, `packages/ts/tests/cases.test.mjs`, `packages/ts/tests/cases.wasm.test.mjs`,
  `packages/php-ext/tests`, `packages/go/tests`다. 생성기가 받지 못하는 target은 원인과 함께 실패하는 target으로
  남는다. 어떤 target도 빠지거나 건너뛰지 않는다(binding BD-27).
- Go 모듈에서 binding은 파일 `zz_generated_*.go`를 소유한다(binding GN-3). Go 모듈은 소스로 받아지므로 이 파일들은
  추적되고, `make bindings`가 다시 쓴다. `packages/ts/wasm/generated`와 `packages/go/internal/ffi/generated`는 추적하지
  않는다. wasm-bindgen은 모듈과 그 glue를 앞의 디렉터리에 쓴다.
  변수 `PYTHON`(3.14 이상), `PHP`, `PHP_CONFIG`, `PHPUNIT`, `GO`, `GOFMT`, `WASM_BINDGEN`이 런타임을 가리킨다.
  `make test-go`는 `packages/go/go.mod`의 go 지시문과 다른 판의 Go를 실패시키고, toolchain을 받지 않도록
  `GOTOOLCHAIN=local`로 실행한다. `make test-wasm`은 `packages/ts/wasm/Cargo.toml`의 `wasm-bindgen` crate와 다른 판의
  wasm-bindgen CLI를 실패시킨다. 그 CLI는 자기 판의 모듈만 읽기 때문이다(binding GN-18). `CARGOFLAGS`는 모든 cargo build에
  인자를 더한다. macOS에서 PHP 확장과 Node addon은 자기 런타임이 그것을 로드할 때 런타임의 기호를
  받는다. 그래서 링커는 그 기호를 정의하지 않은 채 두어야 한다. binding scaffold가 두 crate마다 쓰는 `build.rs`가 자기
  라이브러리에만 그렇게 지시하고(binding GN-3d), 추적되는 어떤 파일도 `RUSTFLAGS`나 `.cargo/config.toml`로 링크 설정을
  주지 않는다. recipe는 Cargo가 만든 라이브러리를 플랫폼의 확장자(macOS `dylib`, Linux `so`)로 가리킨다.
- 도구는 추적되는 파일에 고정되고, binding처럼 checkout의 `var/tools`에 설치된다. `mise.toml`은 Node.js, npm, Go,
  Python, Composer, wasm-bindgen CLI를 binding의 판으로 적고 `mise.lock`은 그 digest를 적는다. `config/toolchain.json`은
  mise의 판과 digest를, `rust-toolchain.toml`은 target `wasm32-unknown-unknown`을 가진 Rust toolchain을, `.php-version`은
  `var/tools/bin`의 wrapper가 실행하는 기계의 PHP를 적는다. 새 checkout은 `make bootstrap`을 실행해 `package.json`의
  `devEngines`가 적는 npm으로 `package-lock.json`을 설치하고, 그다음 `@polyspec/kit`의 설치기를 실행하는 `make install`
  (`make install-tools`)을 실행한다. 모든 recipe는 `var/tools/bin`과 mise의 shim을 `PATH` 앞에 두고 실행한다. `GO`,
  `GOFMT`, `WASM_BINDGEN`은 mise의 shim을, `PHP`와 `PHP_CONFIG`는 `var/tools/bin`의 wrapper를 가리킨다.
- 개발 도구는 이 저장소의 의존성이다. TypeScript와 `@polyspec/kit`은 `package.json`에 있고 `package-lock.json`이 고정한다.
  PHPUnit은 `composer.json`의 `require-dev`에 있고, `composer install`이 `vendor/`에 설치하며,
  `composer.lock`이 고정한다.

## 변경

- 규칙을 바꾸는 변경은 먼저 명세를, 다음에 `contracts/cookies.json`의 case를, 그다음 코드를 바꾼다.
  시험은 자신이 검사하는 규칙 문장(`CK-n.m`)을 밝힌다.
- 규칙은 요구하는 것과 그 근거를 말한다. 근거는 이 라이브러리의 규칙 id, 표준의 절(RFC 6265, RFC
  9110, RFC 9113, draft-ietf-httpbis-rfc6265bis), 또는 브라우저 같은 피어의 동작이다. 추적되는 파일은
  규칙을 어디서 가져왔는지, 누가 요청했는지 적지 않는다.
- 입력에서 아무것도 만들지 않는 부분은 명세의 표 "쿠키를 만들지 않는 입력"의 한 행이며,
  그 규칙, 근거, case를 가진다. 코드는 그 바이트를 버리는 자리에서 규칙을 밝힌다. 행 없이 무시되는 입력은
  결함이다.
- 잘못된 인자는 kind를 가진 오류로 실패한다. 라이브러리는 그 인자를 그럴듯한 값으로 바꾸지 않는다.
- 명세, case, README, 이 파일, 변경 기록(영어와 한국어)은 코드와 같은
  변경에서 바뀐다.
- 추적되는 모든 문서는 영어 파일 `X.md`와 그 한국어 번역 `X.ko.md`이고, 두 파일은 문장 단위로 같은 내용을
  말한다. 그래서 어느 언어의 독자도 같은 규칙을 읽는다. 두 파일은 같은 줄
  `<!-- doc-id: <name> -->`으로 시작하고, 한국어 파일의 둘째 줄은 영어 파일의 sha256을 적은
  `<!-- source-sha256: <hash> -->`이다(`shasum -a 256`). 한 파일을 바꾸는 변경은 다른 파일도 바꾸고 hash를 다시 적는다.
  이 쌍을 검사하는 make target은 없다.

## Rust

- toolchain은 `rust-toolchain.toml`이 고정하는 릴리스다. `make check`는 `-D warnings`를 준 clippy와
  코어의 시험, workspace 전체의 rustfmt(생성된 파일이 필요 없다), `binding check`(계약과
  추적되는 바인딩 패키지)를 실행한다.
- workspace의 lint(루트의 `Cargo.toml`)는 모든 경고, unsafe 코드, `unwrap`, `expect`, `as`
  변환, 써야 하는 값이나 타입을 밝히지 않은 값의 `let _`, 이유 없는 allow attribute를 거부한다.
  `clippy.toml`은 `Option::{unwrap_or, unwrap_or_default, unwrap_or_else, map_or, map_or_else}`와
  `Result::{ok, map_or, map_or_else}`를 금지한다. 규칙에 필요한 사용에는 `#[allow(..., reason = "<rule id>")]`를 붙인다.
- 이 crate는 I/O를 하지 않고 시계, 환경, 난수원을 읽지 않는다(CK-1.2).

## 플랫폼

Linux x64와 arm64(glibc), macOS arm64. Windows는 polyspec의 Rust 코어와 바인딩이 안정되고
ext-php-rs가 고정된 stable toolchain으로 Windows에서 빌드될 때까지 지원하지 않는다.

## Commit

- commit 메시지는 `<type>(<scope>): <Subject>`이고, 사실만 간결하게 적으며, 무엇이 왜 바뀌었는지 말하는 본문을 가진다.
  관련 없는 변경은 별도의 commit이다.
- 추적되는 파일은 Git 커밋을 가리키지 않는다.
