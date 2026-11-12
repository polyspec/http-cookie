# 변경 기록

[English](CHANGELOG.md)

## Unreleased

- 명세 `docs/spec/cookies.md`(CK-1~CK-14), case `contracts/cookies.json`, Rust crate `polyspec-http-cookies`를
  더했다. `parse_cookie`는 요청의 `Cookie` 필드 줄을 순서 있는 이름·값 쌍으로 읽고, `set_cookie`는 `Set-Cookie` 한
  형식을 쓰며 오류 kind는 `invalid_name`, `invalid_cookie_path`, `too_large`, `prefix_not_met`,
  `same_site_none_not_secure`다.
- 초안 인터페이스 계약 `contracts/interface.json`을 더했다. 아직 바인딩은 생성하지 않는다.
- Windows는 지원하지 않는다.
