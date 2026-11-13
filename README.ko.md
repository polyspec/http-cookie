<!-- doc-id: readme -->
<!-- source-sha256: a43bba5806bb2c6b0fb57470da726f32276511e6e5df0adb2b96856a4aff1f4e -->
# http-cookie

[English](README.md)

요청의 `Cookie` 필드 줄에서 쿠키를 읽고, `Set-Cookie` 필드 줄 하나의 필드 값을 쓴다. 구현은 Rust 하나이고 두 연산
모두 순수 함수다. 규칙은 [docs/spec/cookies.ko.md](docs/spec/cookies.ko.md)에, 모든 구현이 통과하는 case는
[contracts/cookies.json](contracts/cookies.json)에 있다.

- 읽기(CK-2~CK-5): 줄을 `;`에서 쌍으로 나누고, 쌍 양끝의 스페이스와 탭을 빼고, 쌍을 첫 `=`에서 나누고, 값을
  percent-decoding하며, 같은 이름은 첫 쿠키가 인정된다. 쿠키가 될 수 없는 쌍(`=` 없음, 빈 이름, UTF-8이 아닌 이름이나
  값)은 쿠키를 만들지 않고, 읽기는 실패하지 않는다.
- 쓰기(CK-6~CK-12): `<name>=<value>; Max-Age=<n>; Path=<path>; Secure; HttpOnly; SameSite=<value>`를 이 순서로,
  호출자가 정한 속성으로 쓴다. 이름은 token이어야 하고, 값은 읽으면 그대로 돌아오도록 percent-encoding하며, 브라우저가
  버릴 쿠키(잘못된 path, 4096바이트 초과, 속성이 모자란 `__Secure-`·`__Host-` 이름, `Secure` 없는 `SameSite=None`)는
  kind를 가진 오류로 실패한다.

## Rust

```rust
use polyspec_http_cookie::{parse_cookie, set_cookie, Attributes, SameSite};

let cookies = parse_cookie(&[b"a=1; b=%C3%A9".to_vec()]);
assert_eq!(cookies, vec![("a".into(), "1".into()), ("b".into(), "é".into())]);

let attributes = Attributes { path: "/".into(), max_age: Some(60), secure: true, http_only: true, same_site: Some(SameSite::Lax) };
assert_eq!(set_cookie("__Host-id", "v", &attributes)?, b"__Host-id=v; Max-Age=60; Path=/; Secure; HttpOnly; SameSite=Lax");
```

## PHP, Python, Node, 브라우저, Go

바인딩은 binding이 인터페이스 계약 [packages/rust/interface.json](packages/rust/interface.json)에서 생성한다(CK-1.5):
Composer 패키지 `polyspec/http-cookie`(PHP 확장 `polyspec_http_cookie`, PHP 8.4 이상), PyPI 패키지
`polyspec-http-cookie`(모듈 `polyspec.http_cookie`, Python 3.14 이상), npm 패키지 `@polyspec/http-cookie`(브라우저는 조건 `browser`와 subpath `./wasm`의 WASM entry를 받는다), Go 모듈
`github.com/polyspec/http-cookie/packages/go`. 언어마다
`parseCookie`/`parse_cookie`, 속성을 이름 있는 인자(PHP)·키워드 인자(Python)·options 객체(Node)로 받는
`setCookie`/`set_cookie`, enum `SameSite`, `kind`와 `status`를 가진 오류 클래스 `HttpCookie<Category>Error`가 있다. 아직
릴리스는 없다.

## 검사

```
make bootstrap       # devEngines의 npm으로 @polyspec/kit을 포함한 package-lock.json 설치
make install         # mise.toml, rust-toolchain.toml, .php-version의 도구를 var/tools에 설치
make check           # 코어: clippy -D warnings, rustfmt --check, 시험; binding check
make bindings        # 바인딩을 packages/*/generated에 생성
make test-bindings   # workspace clippy, 선언의 tsc, 그다음 언어마다 바인딩을 빌드·설치·시험
```

요구 사항: 기계의 rustup과 `.php-version`의 PHP 8.4와 그 `php-config`. `make install`이 `rust-toolchain.toml`의 Rust
릴리스와 `mise.toml`의 도구(Node, Python 3.14, Go, Composer, wasm-bindgen CLI)를 `var/tools`에 설치하고, `composer
install`이 `vendor/`에 PHPUnit을 설치한다(`PYTHON`, `PHP`, `PHP_CONFIG`, `PHPUNIT`, `GO`, `GOFMT`, `WASM_BINDGEN`).
Windows는 아직 지원하지 않는다.
