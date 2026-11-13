
- `scripts/binding.mts`는 Cargo가 고정 커밋으로 만든 binding checkout을 `var/binding/<commit>`에 clone한다. 파일만
  복사하지 않는다. `binding check`는 자기 Git checkout에서 자기 커밋을 읽어 `Cargo.lock`의 커밋과 비교한다(CK-20). 파일
  복사본에서는 커밋을 찾지 못해 `make binding-check`가 실패했다.

- `Cargo.lock`은 같은 라이브러리 묶음의 모든 레포가 고정하는 binding 9de9d03을 고정하고(CK-20), 바인딩 package는 그 scaffold가
  쓰는 것이다. Node 패키지는 addon을 담지 않고 플랫폼 패키지마다 optional dependency를 가진다. `make test-node`는 이
  기계의 플랫폼 패키지를 릴리스처럼 묶어 두 패키지를 함께 설치한다.
