<!-- doc-id: spec-cookies -->
<!-- source-sha256: d528e586c41bab2eaadc27cb788036cd033980f541adf5fd0c7c87af3cd6525e -->
# http-cookie: 규칙

이 문서는 http-cookie의 명세다. 규칙은 목록 항목 `**CK-n**`이다. 규칙의 문장은
anchor `CK-n.m`을 가지고, 시험은 자신이 검사하는 문장을 밝힌다. RFC 6265는 "HTTP State Management Mechanism"이다.
rfc6265bis는 Internet-Draft draft-ietf-httpbis-rfc6265bis-22이며, RFC 6265를 개정하고 지금의 브라우저가
`Set-Cookie` 필드를 어떻게 다루는지 기술한다. 아래에서 rfc6265bis의 절 번호는 그 draft의 절이다.

## 범위

- **CK-1** <a id="CK-1.1"></a> 라이브러리는 요청의 `Cookie` 필드 줄에서 그 요청의 쿠키를 읽고(CK-2~CK-5),
  `Set-Cookie` 필드 줄 하나의 필드 값을 쓴다(CK-6~CK-12). <a id="CK-1.2"></a> 둘 다 순수
  함수다. 시계, 파일, 환경, 난수원을 읽지 않고, 같은 인자는 같은
  결과를 준다. <a id="CK-1.3"></a> 라이브러리는 애플리케이션의 정책을 갖지 않는다. 쿠키 이름, 속성,
  쿠키를 쓸 시점을 고르지 않고, 속성의 기본값이 없다. 호출자가 속성을 하나하나 정한다.
  <a id="CK-1.4"></a> 모든 규칙은 Rust crate `polyspec-http-cookie`(`packages/rust`)에 있다. 바인딩은 타입을
  변환하고 규칙을 갖지 않는다. <a id="CK-1.5"></a> PHP, Python, Node 바인딩은 binding이 인터페이스 계약
  `packages/rust/interface.json`에서 생성한다. 바인딩의 코드는 손으로 쓰지 않는다.

## `Cookie` 필드 읽기

- **CK-2** <a id="CK-2.1"></a> 입력은 한 요청의 `Cookie` 필드 줄들의 필드 값 목록이다. 목록은
  받은 순서를 따르고, 각 필드 값은 받은 그대로의 바이트다. <a id="CK-2.2"></a> `Cookie` 필드가 없는
  요청은 빈 목록을 주고, 빈 목록은 쿠키를 주지 않는다. <a id="CK-2.3"></a> 줄은 하나씩 차례로
  읽으며, 이것은 줄들을 `; `로 이은 것의 쿠키를 준다. user agent는 `Cookie` 줄 하나를 보내고(RFC 6265,
  5.4절), HTTP/2는 자신의 `cookie` 줄들을 `; `로 이으며(RFC 9113, 8.2.3절), 줄의 끝은 `;`처럼 쌍을
  끝내므로 어느 쌍도 줄의 끝을 넘지 않는다. <a id="CK-2.4"></a> host는 받은 줄들을 넘기고
  잇지 않는다. 런타임이 줄들을 이미 한 값으로 이어서 주는 host는 그 값을 한 줄로 넘긴다.
- **CK-3** <a id="CK-3.1"></a> 줄은 `;`(0x3B)마다 쌍으로 나뉜다. <a id="CK-3.2"></a> 쌍 양끝의 스페이스(0x20)와
  탭(0x09)은 제거된다. 쌍 안쪽의 그런 바이트는 유지된다. <a id="CK-3.3"></a> 쌍은 자신의
  첫 `=`(0x3D)에서 그 앞의 바이트인 이름과 그 뒤의 바이트인 값으로 나뉜다. 뒤에 오는 `=`는 값의 바이트다.
- **CK-4** <a id="CK-4.1"></a> 이름은 decoding하지 않는다. 이름의 바이트가 이름의 텍스트다. <a id="CK-4.2"></a> 값은
  percent-decoding한다. `%`와 그 뒤의 16진 숫자 둘(대소문자 무관)은 그 숫자가 나타내는 바이트이고, 그 밖의 모든
  바이트는 그 자신이다. 여기에는 `+`, 16진 숫자 둘이 뒤따르지 않는 `%`, 큰따옴표, 쉼표가 포함된다.
  <a id="CK-4.3"></a> 값 맨 앞의 U+FEFF는 값의 문자이고, 큰따옴표로 둘러싼 값은 그
  따옴표를 유지한다. <a id="CK-4.4"></a> `=`가 없는 쌍은 쿠키를 주지 않으며, 빈 쌍도 그렇다. <a id="CK-4.5"></a> 이름이
  빈 쌍은 쿠키를 주지 않는다. <a id="CK-4.6"></a> 이름이 유효한 UTF-8이 아닌 쌍은 쿠키를 주지 않는다.
  <a id="CK-4.7"></a> decoding한 값이 유효한 UTF-8이 아닌 쌍은 쿠키를 주지 않는다. <a id="CK-4.8"></a> RFC 6265는
  쿠키 값에 `cookie-octet`(4.1.1절) 밖의 바이트를 위한 encoding을 주지 않으므로, 값에는 encoding이 하나 필요하다.
  CK-8의 쓰기는 percent-encoding(RFC 3986, 2.1절)을 쓰고, 쿠키 값을
  `encodeURIComponent`로 쓰는 브라우저 script는 이 decoding이 그대로 되읽는 바이트를 준다. `+`는 `+`로 남는다. 스페이스를 뜻하는 `+`는
  percent-encoding이 아니라 `application/x-www-form-urlencoded`에 속하기 때문이다. 다른 쓰기 주체가 문자 그대로의 `%`와
  16진 숫자 둘로 쓴 쿠키는 모든 쿠키와 같이 decoding되어 읽힌다. 읽는 쪽은 쿠키를 누가 썼는지 알 수 없으므로
  하나의 decoding을 모든 쿠키에 적용한다.
- **CK-5** <a id="CK-5.1"></a> 요청의 쿠키는 이름에서 값으로 가는 순서 있는 map이고, 순서는
  각 이름이 처음 쿠키를 준 순서다. <a id="CK-5.2"></a> 쿠키를 주는 쌍 가운데 한 이름의 첫 쌍이 인정되고,
  그 이름의 뒤 쌍은 쿠키를 주지 않는다. 쿠키를 주지 않는 쌍(CK-4.4~CK-4.7)은 인정되지 않으므로, 그 이름의
  뒤 쌍이 인정된다. <a id="CK-5.3"></a> RFC 6265, 5.4절, 2단계는 user agent가 path가 긴
  쿠키를 먼저 보내게 하므로, 한 이름의 첫 쿠키는 가장 구체적인 path의 쿠키다. RFC 6265, 4.2.2절은
  서버에게 쿠키의 순서에 기대지 말라고 한다. user agent는 길이가 같은 path를 가진 같은 이름의 쿠키 둘을
  보낼 수 있고, 그때는 첫 쿠키가 인정된다. <a id="CK-5.4"></a> 읽기는 실패하지 않는다. 모든 바이트 문자열 목록은 map을 준다.

### 쿠키를 만들지 않는 입력

`Cookie` 필드는 브라우저가 요청의 host와 path에 대해 보관하는 모든 쿠키를 담으며, 같은 host의
다른 애플리케이션이나 페이지의 script가 쓴 쿠키도 담는다. 그러므로 이 읽기가 쿠키로 줄 수 없는 쌍은
요청의 잘못이 아니고, 그런 쌍 때문에 필드 전체를 실패시키면 그 host에 쓰는 누구든
모든 쿠키의 읽기를 막을 수 있다. 아래의 각 행은 입력에서 아무것도 주지 않는 부분이다. case는 그 부분이
다른 쿠키를 바꾸지 않음을 보인다. 코드는 그 바이트를 버리는 자리에서 규칙을 밝힌다.

| 규칙 | 아무것도 주지 않는 바이트 | 이유 | Case |
|---|---|---|---|
| CK-3.2 | 쌍 양끝의 스페이스와 탭 | RFC 6265, 5.4절, 4단계는 user agent가 쌍 사이에 `; `를 쓰게 한다. 5.2절은 `Set-Cookie`의 이름과 값 둘레의 스페이스와 탭을 제거하므로, 그 바이트는 이름의 부분도 값의 부분도 아니다 | `spaces_and_tabs_at_the_start_of_a_pair_are_removed`, `spaces_and_tabs_at_the_end_of_a_pair_are_removed` |
| CK-4.4 | `=`가 없는 쌍(빈 쌍 포함) | 이름이 빈 쿠키는 그 값만으로 보내지므로(rfc6265bis, 5.8.3절, 6단계), 그런 쌍은 이름 없는 쿠키이거나 아무것도 아니다. map에는 이름 없는 항목이 없고, rfc6265bis, 4.1.1절은 서버가 이름 없는 쿠키를 만드는 것을 금지한다 | `a_pair_without_an_equals_sign_gives_no_cookie`, `empty_pairs_give_no_cookie` |
| CK-4.5 | 이름이 빈 쌍 | CK-4.4와 같다: 이름 없는 쿠키 | `a_pair_with_an_empty_name_gives_no_cookie` |
| CK-4.6 | 이름이 유효한 UTF-8이 아닌 쌍 | 결과의 이름은 텍스트다. 그 바이트를 대체하면 다른 쿠키의 이름이 된다 | `a_name_that_is_not_utf8_gives_no_cookie` |
| CK-4.7 | decoding한 값이 유효한 UTF-8이 아닌 쌍 | 결과의 값은 텍스트다. 대체 문자는 다른 값을 준다 | `a_value_that_is_not_utf8_gives_no_cookie_and_a_later_pair_counts` |
| CK-5.2 | 이미 쿠키를 준 이름의 뒤 쌍 | CK-5.3: 첫 쿠키가 가장 구체적인 path의 쿠키다 | `the_first_cookie_of_a_name_counts` |

## `Set-Cookie` 필드 쓰기

- **CK-6** <a id="CK-6.1"></a> `set_cookie(name, value, attributes)`는 `Set-Cookie` 필드 줄 하나의 필드 값을
  돌려주며(RFC 6265, 4.1절), 그 값은 ASCII다. <a id="CK-6.2"></a> 속성은 `path`(텍스트), `max_age`(음이
  아닌 정수 또는 없음), `secure`(boolean), `http_only`(boolean), `same_site`(`Strict`, `Lax`,
  `None` 또는 없음)다. <a id="CK-6.3"></a> `path`, `secure`, `http_only`에는 기본값이 없다(CK-1.3). `max_age`가 없으면
  `Max-Age`를 쓰지 않으며, 그러면 브라우저 세션과 함께 끝나는 쿠키가 된다(RFC 6265, 5.3절, 3단계).
  `same_site`가 없으면 `SameSite`를 쓰지 않는다. <a id="CK-6.4"></a> 검사는 다음 순서로 실행되고, 처음 실패한 검사가
  오류다: 이름(CK-7), path(CK-10), 크기(CK-11.3), prefix(CK-12), `SameSite=None`(CK-12.4).
- **CK-7** <a id="CK-7.1"></a> 이름은 `token`이다(RFC 6265, 4.1.1절, `cookie-name`; RFC 9110, 5.6.2절).
  곧 문자 `!`, `#`, `$`, `%`, `&`, `'`, `*`, `+`, `-`, `.`, `^`, `_`, `` ` ``, `|`, `~`, ASCII
  숫자, ASCII 글자 가운데 하나 이상이다. <a id="CK-7.2"></a> 그 밖의 이름은, 빈 이름도 포함해, kind
  `invalid_name`과 메시지 `a cookie name is not a token (CK-7)`로 실패한다. <a id="CK-7.3"></a> 이름은 있는 그대로 쓴다.
  읽는 쪽이 이름을 decoding하지 않기 때문이다(CK-4.1).
- **CK-8** <a id="CK-8.1"></a> 값은 어떤 텍스트든 된다. <a id="CK-8.2"></a> 값은 그 UTF-8 바이트로 쓰되, `%`와
  `cookie-octet`(RFC 6265, 4.1.1절: 0x21, 0x23~0x2B, 0x2D~0x3A, 0x3C~0x5B,
  0x5D~0x7E)이 아닌 모든 바이트는 `%`와 그 바이트의 대문자 16진 숫자 둘로 쓴다. <a id="CK-8.3"></a> 그러므로
  쓴 값은 `cookie-value`이고, 그 값을 CK-4로 읽으면 원래 텍스트가 돌아온다. 빈 텍스트는
  빈 값으로 쓴다.
- **CK-9** <a id="CK-9.1"></a> 필드 값은 `<name>=<value>`이고, 이어서 `max_age`가 있으면
  `; Max-Age=<max_age>`, `; Path=<path>`, `secure`가 true이면 `; Secure`, `http_only`가 true이면 `; HttpOnly`,
  `same_site`가 있으면 `; SameSite=<same_site>`가 온다. <a id="CK-9.2"></a> 이것은
  `cookie-av`(rfc6265bis, 4.1.1절)의 선택지 순서이고, 속성 이름은 그 문법의 대소문자를 따른다. user agent는
  속성을 어떤 순서로든 읽으므로(RFC 6265, 5.2절), 고정된 순서 하나가 한 쿠키에 한 형태를 준다.
  <a id="CK-9.3"></a> 라이브러리는 `Expires`, `Domain`, 그 밖의 속성을 쓰지 않는다. `Max-Age`만으로
  수명이 정해지고(RFC 6265, 5.3절, 3단계는 `Expires`보다 `Max-Age`를 우선한다), `Expires` 날짜에는 시계가 필요한데 순수
  함수에는 시계가 없다(CK-1.2). `Domain`이 없는 쿠키는 host-only다(RFC 6265, 5.3절, 6단계). 그 쿠키의 host만
  그 쿠키를 설정하고 받으며, `__Host-`가 이것을 요구한다(CK-12.2).
- **CK-10** <a id="CK-10.1"></a> path는 `/`로 시작하고, path의 모든 문자는 `;`를 뺀 0x20~0x7E이며
  (RFC 6265, 4.1.1절, `path-value`), path는 1024자 이하다(rfc6265bis, 5.6절과 5.7절: user
  agent는 더 긴 속성 값을 무시한다). <a id="CK-10.2"></a> 그 밖의 path는 kind `invalid_cookie_path`와
  메시지 `a cookie path is not an absolute path of at most 1024 printable ASCII characters without ';' (CK-10)`로 실패한다.
  <a id="CK-10.3"></a> user agent는 `/`로 시작하지 않는 path를 요청의 기본 path로 바꾸고
  (RFC 6265, 5.2.4절), 1024 octet보다 긴 path를 무시한다. 그러므로 둘 중 어느 것을 써도
  호출자가 정한 것과 다른 path의 쿠키가 된다.
- **CK-11** <a id="CK-11.1"></a> `Max-Age`는 앞의 0 없이 10진수로 쓴다. <a id="CK-11.2"></a> `Max-Age=0`은
  쿠키를 제거한다. user agent는 0 이하의 `Max-Age`에 가장 이른 만료 시각을 주므로(RFC 6265, 5.2.2절),
  호출자는 쿠키의 이름, path, 빈 값, `max_age` 0으로 쿠키를 제거한다.
  `max-age-av`의 문법(RFC 6265, 4.1.1절)은 0이 아닌 숫자로 시작한다. 그래도 라이브러리는 `0`을 쓴다.
  5.2.2절의 user agent 규칙이 0을 정의하고, 0이 `Expires` 없이 쿠키를 제거하는 유일한 방법이기 때문이다(CK-9.3).
  user agent는 자신이 유지하는 age를 제한한다(rfc6265bis, 5.5절, 최대 400일). user agent는 쿠키를
  더 짧은 age로 저장하므로, 라이브러리는 주어진 수를 쓴다. <a id="CK-11.3"></a> 이름과 쓴 값(CK-8.2)을
  합쳐 4096바이트를 넘으면 kind `too_large`와 메시지
  `a cookie name and value are longer than 4096 bytes (CK-11)`로 실패한다. user agent가 그런 쿠키를 통째로 무시하기 때문이다
  (rfc6265bis, 5.6절과 5.7절).
- **CK-12** <a id="CK-12.1"></a> ASCII 대소문자를 구별하지 않고 비교해 `__Secure-`로 시작하는 이름은
  `secure`를 요구한다(rfc6265bis, 4.1.3.1절). <a id="CK-12.2"></a> ASCII 대소문자를 구별하지 않고
  비교해 `__Host-`로 시작하는 이름은 `secure`와 path `/`를 요구한다. 그 쿠키에는 `Domain`이 없는데, 라이브러리는 `Domain`을 쓰지 않는다(CK-9.3)
  (rfc6265bis, 4.1.3.2절). <a id="CK-12.3"></a> 자기 prefix의 조건을 채우지 못한 이름은
  kind `prefix_not_met`과 메시지 `a cookie name with the prefix __Secure- or __Host- lacks the attributes
  that the prefix requires (CK-12)`로 실패한다. user agent는 prefix를 대소문자 구별 없이 비교하고(rfc6265bis, 5.4절),
  자기 prefix의 조건을 채우지 못한 쿠키를 통째로 무시하므로(5.7절, 20단계와 21단계),
  라이브러리는 user agent가 비교하는 대로 비교한다. <a id="CK-12.4"></a> `same_site` `None`은 `secure`를 요구한다. 그렇지 않으면
  호출은 kind `same_site_none_not_secure`와 메시지
  `a cookie with SameSite=None lacks Secure (CK-12)`로 실패한다. user agent가 그런 쿠키를 통째로 무시하기 때문이다(rfc6265bis,
  5.7절, 19단계). <a id="CK-12.5"></a> 라이브러리는 이런 쿠키를 쓰지 않고 실패한다. user agent가
  서버에 아무 신호 없이 그 쿠키를 버릴 것이기 때문이다.

## 오류

- **CK-13** <a id="CK-13.1"></a> 모든 실패는 crate의 오류 enum의 오류이고, kind를 가지며, HTTP status가 없고,
  그 규칙이 정한 메시지를 가진다. 바인딩은 메시지를 그대로 준다. <a id="CK-13.2"></a> kind는
  `invalid_name`(CK-7), `invalid_cookie_path`(CK-10), `too_large`(CK-11.3), `prefix_not_met`(CK-12.3),
  `same_site_none_not_secure`(CK-12.4)이고, 모두 category `value`다. path의 kind는 `invalid_path`가 아니다. `invalid_path`는 파일 시스템 경로를 거부하는 바인딩의 kind이고, 한 kind는 한 실패를 가리켜야 하기 때문이다. <a id="CK-13.3"></a> 실패는 요청의 오류가 아니라
  인자를 준 호출의 오류이므로, 어느 kind도 HTTP status를 갖지 않는다. <a id="CK-13.4"></a> 읽기(CK-5.4)에는
  오류가 없다.

## 검증

- **CK-14** <a id="CK-14.1"></a> `contracts/cookies.json`은 두 연산의 case를 담는다: CK-2~CK-5의
  group `parse`와 CK-6~CK-12의 group `set_cookie`. case는 자신이 검사하는 문장을 밝힌다.
  <a id="CK-14.2"></a> case의 바이트 문자열은 JSON 텍스트이고, 그 문자 U+0000~U+00FF가 바이트다. 실패는
  `kind`, `status`, `category`, `message`를 가진 key `raises`이고, 그런 case에는 결과 key가 없다.
  <a id="CK-14.3"></a> crate는 모든 case를 돌린다. 또한 문법의 자리마다 모든 바이트 값을 검사하고(`Cookie`의
  이름의 바이트와 값의 바이트, `Set-Cookie`의 이름, 값, path의 바이트), 문자열의 경계를 검사한다:
  값의 끝에 있는 `%`와 숫자 하나가 따르는 `%`, 줄의 처음과 끝에 있는 쌍, `;`의 앞과
  뒤에서 나뉜 줄, path의 크기 1024와 1025, 이름과 값의 크기 4096과 4097.
  <a id="CK-14.4"></a> property 시험은 무작위 입력에 대해, 줄 목록을 읽은 결과가 그 줄들을
  `; `로 이어 읽은 결과와 같고(CK-2.3), 읽기가 panic하지 않으며(CK-5.4), 쓴 쿠키를 읽으면 그 이름과 값을 준다는 것을
  검사한다(CK-8.3).
  <a id="CK-14.5"></a> 각 바인딩은 설치된 그대로의 자기 패키지를 통해(가상 환경의
  Python wheel, pack해서 설치한 npm 패키지, PHP에 로드한 PHP 확장) 자기 언어의
  runner(unittest, `node --test`, PHPUnit)로 모든 case를 돌린다. 결과는 case의 결과와 같고, 실패는 그 언어에서
  자기 category의 class, kind, status, 그리고 case가 메시지를 정했으면 그 메시지를 가진다. binding이
  그 바인딩을 위해 생성하는 시험이 그 옆에서 함께 돈다.
