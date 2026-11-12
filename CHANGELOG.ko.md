# 변경 기록

[English](CHANGELOG.md)

## Unreleased

- 명세 `docs/spec/cookies.md`(CK-1~CK-14), case `contracts/cookies.json`, Rust crate `polyspec-http-cookies`를
  더했다. `parse_cookie`는 요청의 `Cookie` 필드 줄을 순서 있는 이름·값 쌍으로 읽고, `set_cookie`는 `Set-Cookie` 한
  형식을 쓰며 오류 kind는 `invalid_name`, `invalid_cookie_path`, `too_large`, `prefix_not_met`,
  `same_site_none_not_secure`다.
- 초안 인터페이스 계약 `contracts/interface.json`을 더했다. 아직 바인딩은 생성하지 않는다.
- 인터페이스 계약을 `packages/rust/interface.json`으로 옮기고 crate의 `include`에 넣었다. crate에 의존하는 레포가
  Cargo가 해석한 패키지에서 계약을 읽게 하기 위해서다. 계약은 bindgen 스키마를 따른다: category 오류 클래스는 모든
  언어에서 `Cookies<Category>Error`(`CookiesValueError` 등)이고, 속성 `max_age`와 `same_site`는 기본값을 가진 field
  대신 호출자가 적는 optional field이며 속성이 없으면 `null`이다(CK-1.3, CK-6.3).
- Windows는 지원하지 않는다.
