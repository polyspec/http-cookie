<!-- doc-id: agents -->
<!-- source-sha256: 2d827ae5302fb828c4be8f5f2569cc06805d1bcd53241cf886bfb1aa1d56586c -->
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
  `make binding-check`가 실패한다. 각 바인딩의 `generated/`(`packages/go/internal/ffi/generated`와
  `packages/ts/wasm/generated` 포함)는 `make bindings`가 쓰고 추적되지 않는다. Go 모듈에서 binding은 디렉터리 대신
  `zz_generated_*.go` 파일을 소유하고, 이 파일들은 추적된다(binding GN-3). `go get`은 모듈을 추적된 파일에서 받기
  때문이다. `make bindings`가 이 파일들을 다시 쓴다. 각 바인딩의 `tests/`는 그 언어로 contract case를 돌리는 runner를 담는다(CK-14.5).
- `contracts/cookies.json`: 규칙의 case(CK-14).
- `scripts/binding.mts`: binding을 실행한다("바인딩" 절을 본다).
- `docs/spec/`: 명세. 루트: `AGENTS.md`, `README.md`, `CHANGELOG.md`와 그 한국어 번역
  (`*.ko.md`), `Makefile`, `rust-toolchain.toml`, `clippy.toml`, `rustfmt.toml`, `.cargo/config.toml`, 그리고
  개발 도구의 `package.json`, `package-lock.json`, `composer.json`, `composer.lock`.
- 바인딩 옆에 지우거나 남겨 둘 다른 언어의 구현은 없다.

## 바인딩

- `polyspec-binding-core`는 binding이 릴리스될 때까지 코어와 바인딩 crate의 Git 의존성이다(binding 레포의
  `v1/rust-core` 브랜치). `Cargo.lock`이 그 커밋을 고정한다. 코어는 자기 오류에
  `binding_core::Kinds`를 구현한다(CK-13.2, CK-13.3).
- binding 자체는 같은 레포의 TypeScript 도구이고, 아직 어느 registry에도 없다. `scripts/binding.mts`는
  `Cargo.lock`이 고정한 커밋으로 Cargo가 만든 checkout에서 binding을 가져와 `var/binding/<commit>`에 clone하고,
  거기에 binding의 `package-lock.json`대로 의존성을 설치한다. 그래서 생성기와 `binding_core`가 한 판이다.
  이 clone은 그 커밋의 binding Git checkout이다. binding check는 이 checkout에서 자기 커밋을 읽어
  `Cargo.lock`의 커밋과 비교하며(CK-20), 파일만 복사한 것은 이 비교에서 실패한다. 이것은 도구를 설치하는 보통의
  방법이 아니고 binding의 첫 릴리스까지만 유효하다. 그 뒤에는 binding이 `package-lock.json`이
  고정하는 npm 패키지가 되고, 이 script는 없어진다.
- `make bindings`가 생성한다. 바인딩을 빌드하거나 시험하는 모든 make target은 먼저 이것을 실행한다. `make test-bindings`는
  workspace 전체에 clippy를, 이어서 `make check-declarations`(생성된 `index.d.ts`에 `tsc --noEmit --strict`)를, 그다음
  `make test-python`(maturin이 빌드해 `var/venv`에 설치한 wheel), `make test-node`(생성된 entry 옆의
  addon, pack해서 `var/node`에 설치한 패키지), `make test-php`(`PHP_CONFIG`에 맞춰 빌드해
  `PHP`에 로드한 확장, `PHPUNIT`으로 돌리는 시험)를 실행한다. 각각은 binding이 생성하는 시험과
  contract case를 돌린다. Go와 WASM 바인딩을 빌드하거나 시험하는 make target은 아직 없고, binding의
  end-to-end 검사(binding의 `make e2e`)만 그것들을 돌린다. 변수 `PYTHON`(3.14 이상), `PHP`(8.4 이상), `PHP_CONFIG`, `PHPUNIT`이
  런타임을 가리키고, `CARGOFLAGS`는 모든 cargo build에 인자를 더한다. macOS에서 PHP 확장과 Node addon은 런타임이
  자신을 로드할 때 그 런타임의 기호를 받는다. 그래서 `.cargo/config.toml`이 링커에게 그 기호를 정의하지 않은 채 두게 하고,
  recipe는 Cargo가 빌드하는 라이브러리를 플랫폼의 확장자(macOS `dylib`, Linux `so`)로 가리킨다.
- 개발 도구는 레포의 의존성이다: TypeScript는 `package.json`에 있고
  `package-lock.json`이 고정하며, PHPUnit은 `composer.json`의 `require-dev`에 있고 `vendor/`에 설치된다(`PHPUNIT`의
  기본값). `composer.lock`이 PHPUnit과 그 의존성을 고정하고, `composer install`이 그것들을 설치한다.

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
