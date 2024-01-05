해당 dockerfile은 confluent-hub install 명령어가 프록시 환경에서는 적용되지 않기 때문입니다.
따라서 미리 confluent-hub에서 다운로드 완료한 이미지를 이용해 작업을 진행합니다.

이 dockerfile은 미리 필요한 connector를 설치한 이미지를 만드는 코드입니다.