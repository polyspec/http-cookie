# http-cookies

[English](README.md)

요청의 `Cookie` 필드 줄에서 쿠키를 읽고, `Set-Cookie` 필드 줄 하나의 필드 값을 쓴다. 구현은 Rust 하나이고 두 연산
모두 순수 함수다. 규칙은 [docs/spec/cookies.md](docs/spec/cookies.md)에, 모든 구현이 통과하는 case는
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
use polyspec_http_cookies::{parse_cookie, set_cookie, Attributes, SameSite};

let cookies = parse_cookie(&[b"a=1; b=%C3%A9".to_vec()]);
assert_eq!(cookies, vec![("a".into(), "1".into()), ("b".into(), "é".into())]);

let attributes = Attributes { path: "/".into(), max_age: Some(60), secure: true, http_only: true, same_site: Some(SameSite::Lax) };
assert_eq!(set_cookie("__Host-id", "v", &attributes)?, b"__Host-id=v; Max-Age=60; Path=/; Secure; HttpOnly; SameSite=Lax");
```

PHP, Python, Node 바인딩은 초안인 [contracts/interface.json](contracts/interface.json)에서 생성할 예정이며, 아직 바인딩은
없다.

## 검사

```
make check   # clippy -D warnings, rustfmt --check, 시험
```

요구 사항: `rust-toolchain.toml`의 Rust 릴리스. Windows는 아직 지원하지 않는다.
