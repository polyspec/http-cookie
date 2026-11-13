<!-- doc-id: changelog -->
<!-- source-sha256: 15f6e654a687e3414c19445476a98ad1569d7336fe9f72da48fb2943e3b55cac -->

- binding이 생성하는 Go 파일을 추적하고 Go와 WASM의 다른 생성 디렉터리는 무시한다: `packages/go/zz_generated_*.go`,
  `packages/go/errors/`, `packages/go/internal/binding/`에는 `make bindings`가 쓰는 `zz_generated_*.go` 파일이 있고,
  binding은 Go 모듈에서 이 파일들을 소유하며 이 파일들은 추적된다(binding GN-3). `go get`이 모듈을 추적된 파일에서
  받게 하기 위해서다. `.gitignore`는 `packages/go/internal/ffi/generated/`와 `packages/ts/wasm/generated/`를 적는다.
  전에는 `make bindings`가 이 파일과 디렉터리를 추적되지도 무시되지도 않은 채 남겼다. binding의 end-to-end check
  (`BD-27`을 합친 commit에서 binding의 `make e2e`)는 계약의 다섯 target을 각 runner에서 돌려 모두 통과한다: PHP는
  PHPUnit, Python은 unittest, Node와 WASM은 `node --test`, Go는 `go test`이고, 건너뛴 시험은 없다.

- 계약이 binding의 다섯 target을 적는다(BD-27): `packages/rust/interface.json`의 `targets`는 php, python, node,
  go, wasm이고, `packages.go`는 Go 모듈 `github.com/polyspec/http-cookie/packages/go`를 적으며, npm 패키지의 entry
  `.`는 조건 `{node: napi, browser: wasm}`을 받고, 각 오류 클래스는 WASM 클래스로 Node의 것을 적으며,
  `errors.fixed`는 Node처럼 WASM에도 `TypeError`를 적는다. 전에는 계약이 php, python, node만 적어 Go와 WASM 바인딩이
  생성되지 않았다. 추적되는 바인딩 패키지에 binding scaffold가 두 target을 위해 쓰는 것(`packages/go/go.mod`,
  `packages/go/internal/ffi`, `packages/ts/wasm`, `packages/ts/package.json`의 브라우저 exports)이 더해지고, 두
  crate가 workspace의 member가 된다. 계약 검사의 finding은 없다. Go와 WASM 바인딩을 빌드하거나 시험하는 make target은
  아직 없다. `make fmt-check`는 `packages/go/internal/ffi/src/lib.rs`에서 실패한다: binding scaffold가 그 `allow`
  속성을 125자의 한 줄로 쓰는데 `rustfmt.toml`의 `max_width = 120`을 넘고, 이 파일은 손으로 고치지 않는다(L-2).

- `Cargo.lock`은 값 alias, 실패 주입, 오류 unit의 생성기를 한 개발 줄기에 가진 binding을 고정하고(CK-20), 바인딩
  package는 그 scaffold가 쓰는 것이다. 각 바인딩 crate는 `polyspec-binding-core`를 feature `construct`와 `lock`과 함께
  받고, lock에는 `polyspec-binding-core`의 의존으로 `tokio`가 더해진다. 계약 검사의 지적은 없다. 생성된 소스에는
  member가 내는 오류 variant가 모두 `Error`의 variant임을 컴파일할 때 증명하는 코드가 더해지고(E-5), 생성된 Node
  시험은 발생한 오류마다 그 class의 `instanceof`도 검사한다. 세 package의 동작은 같다.

- `make test-php`와 `make lint-bindings`가 인자 없이 PHP를 찾는다. `PHP`와 `PHP_CONFIG`의 기본값은 `command -v`가
  주는 경로다. 전에는 기본값이 이름 `php`뿐이었고, ext-php-rs는 그것을 파일의 경로로 읽어
  "php executable not found at php"로 실패했다.

- 추적되는 모든 문서가 한국어 번역과 쌍의 marker를 가진다. `AGENTS.ko.md`와
  `docs/spec/cookies.ko.md`를 새로 두었고, 모든 문서는 자신의 `doc-id`로 시작하며, 한국어 파일은 자신의 영어 파일의
  sha256을 적는다(`source-sha256`). `AGENTS.md`가 이 규칙을 말한다.

- `composer.lock`을 추적한다. 그래서 `composer install`이 PHP 확장의 시험을 돌리는 PHPUnit을 고정된 한
  판으로 설치한다.

- 바인딩이 macOS에서 빌드되고 로드된다. `.cargo/config.toml`이 macOS 링커에게 PHP와 Node의 기호를 정의하지 않은 채
  두게 한다. 그 기호는 런타임이 라이브러리를 로드할 때 준다. `make test-node`와 `make test-php`는 Cargo가 만든
  라이브러리를 플랫폼의 확장자(macOS `dylib`, Linux `so`)로 가리킨다. 전에는 macOS에서 PHP 확장이 링크되지 않았고, 두
  recipe가 Cargo가 거기서 만들지 않는 `.so` 파일을 가리켰다.

- `scripts/binding.mts`는 Cargo가 고정 커밋으로 만든 binding checkout을 `var/binding/<commit>`에 clone하고,
  그 파일을 더는 복사하지 않는다. `binding check`는 자기 Git checkout에서 자기 커밋을 읽어 `Cargo.lock`의
  커밋과 비교한다(CK-20). 파일 복사본에서는 커밋을 찾지 못했고 `make binding-check`가 실패했다.

- `Cargo.lock`은 같은 라이브러리 묶음의 모든 레포가 고정하는 커밋인 binding 9de9d03을 고정하고(CK-20),
  바인딩 패키지는 그 scaffold가 쓰는 것이다. Node 패키지는 addon을 담지 않고 플랫폼 패키지마다 optional dependency
  하나를 가진다. `make test-node`는 이 기계의 플랫폼 패키지를 릴리스가 하듯이 pack하고 두 패키지를
  설치한다.
