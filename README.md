# 드워프 포트리스(Dwarf Fortress) 한국어 번역 프로그램

### [여기에서 다운로드](https://github.com/refracta/dfint-rust-cjk-ko/release) 최신 한국어 번역 프로그램 전체 패키지

이 프로젝트는 Viz가 상위 프로젝트 [df-steam-hook-rs](https://github.com/dfint/df-steam-hook-rs)를 기반으로 개발한 [드워프 포트리스 중국어 번역 프로그램](https://gitee.com/vizv/dfint-rust-cjk)을 수정하여 만들어졌습니다. 

* 본 프로젝트는 자유 소프트웨어이며 소스 코드는 [GitHub:refracta/dfint-rust-cjk-ko](https://github.com/refracta/dfint-rust-cjk-ko)에 호스팅됩니다.
* 프로젝트 번역 파일은 [GitHub:refracta/df-translations](https://github.com/refracta/df-translations)에 호스팅됩니다.

## 호환 버전

공식 채널에서 구매한 디럭스(Premium Edition, 유료/그래픽 버전) 게임 본체 중 아래 버전만 호환됩니다.

* Linux/Windows [**53.XX+** Steam 버전](https://store.steampowered.com/app/975370/Dwarf_Fortress/)
* Linux/Windows [**53.XX+** itch.io 버전](https://kitfoxgames.itch.io/dwarf-fortress)
* Linux/Windows [**53.XX+** 클래식 버전](http://www.bay12games.com/dwarves/)

**주의 사항:**

* [클래식 버전](http://www.bay12games.com/dwarves/) (Classic Edition, 무료/문자(ASCII) 버전)은 동작할 수는 있지만 지원하지 않으며, itch.io 버전으로 표시될 수도 있습니다.
* 실행 파일을 수정한 게임 버전은 동작할 가능성은 있어도 지원하지 않습니다. 한국어 번역 프로그램은 실행 파일 내의 여러 메모리 주소 오프셋에 의존하므로, 앞으로도 지원할 계획이 없습니다.
* 모험 모드는 일부 한국어 번역이 될 수 있으나 지원하지 않습니다. 
* 이 한국어 번역 프로그램은 게임 본체를 포함하지 않습니다. [itch.io](https://kitfoxgames.itch.io/dwarf-fortress) 또는 [Steam](https://store.steampowered.com/app/975370/Dwarf_Fortress/)에서 별도로 구매하세요.

## 사용 방법

1. 드워프 포트리스 세이브를 백업합니다.
2. 드워프 포트리스 버전이 지원되는 53.XX 이상인지 확인합니다.
3. [여기에서](https://github.com/refracta/dfint-rust-cjk-ko/release) 최신 한국어 번역 프로그램 전체 패키지를 다운로드합니다.
4. 압축을 풀고, 안의 모든 파일/폴더를 게임 루트 디렉터리에 복사합니다.
5. 게임을 평소대로 실행합니다.
6. 게임 내에서 `Ctrl+F2` 단축키로 한국어 번역을 켜고/끌 수 있습니다.

## 알려진 문제

* 한국어 번역 프로그램은 게임 성능(프레임)을 저하시킬 수 있습니다.
* 일부 텍스트가 겹치거나, 하위 레이어 텍스트가 상위 레이어로 비쳐 보이는 경우가 있습니다.
* 라벨 너비 렌더링이 올바르지 않을 수 있습니다.
* 번역이 누락된 텍스트가 많습니다.
* 가운데 정렬이 필요한 번역이 가운데 정렬되지 않은 경우가 있습니다.
* 생략 부호(「…」로 끝나는 텍스트)가 포함된 경우, 현재는 올바르게 매칭/번역하지 못합니다.
* 기본 설정에서 dfint 구(舊) 사전을 활성화하기 때문에, 일부 번역이 부정확하거나 부분적으로만 번역될 수 있습니다.
* 다의어가 포함된 조합은 오역될 수 있습니다(예: `ash logs`가 ‘물푸레나무 원목’이 아니라 ‘재(灰) 원목’으로, `pig iron`이 ‘선철’이 아니라 ‘돼지 철 주괴’로 번역되는 경우 등).

## 감사의 말

* 상위 프로젝트 [dfint-rust-cjk](https://gitee.com/vizv/dfint-rust-cjk)의 중국어 번역 프레임워크
* 최상위 프로젝트 [df-steam-hook-rs](https://github.com/dfint/df-steam-hook-rs)의 hook 프레임워크
* dfint 구(舊) 사전: [dfint](https://github.com/dfint/autobuild/blob/main/translation_build/csv/Korean/dfint_dictionary.csv)
* 폰트: [Noto CJK](https://github.com/notofonts/noto-cjk)
* 리버스 엔지니어링 스크립트: [DFHack의 df_misc 저장소](https://github.com/DFHack/df_misc)
* DFHack의 [dfhooks API chainloader](https://github.com/DFHack/dfhooks)
## 라이선스

* 본 한국어 번역 프로그램의 저작권은 상위 프로젝트와 동일하게 MIT 라이선스로 배포됩니다. 
* Noto CJK 폰트는 SIL Open Font License로 배포됩니다: [GitHub:notofonts/noto-cjk:Sans/LICENSE](https://github.com/notofonts/noto-cjk/blob/main/Sans/LICENSE)
* 사용된 오픈소스 라이브러리는 `Cargo.toml` 및 `Cargo.lock`을 참고하세요(각 라이브러리 저장소의 라이선스 정보 포함).

## 빌드

1. [Rust 프로그래밍 언어 / 시작하기 / 설치](https://rustwiki.org/zh-CN/book/ch01-01-installation.html) 문서를 참고해 rustup 및 Rust/C++ 툴체인을 설치합니다.
2. nightly 버전 설치: `rustup install nightly`
3. 한국어 번역 프로그램 빌드: `cargo build --release`
4. `target/release`의 `libdfint_hook.so` 또는 `dfint_hook.dll`을 게임 디렉터리에 복사한 뒤, `libdfhooks.so` 또는 `dfhooks.dll`로 이름을 바꿉니다.
5. `data/offsets.txt`를 게임 디렉터리의 `dfint-data/offsets.txt`로 복사합니다.
6. `config.txt.example`을 게임 디렉터리의 `dfint-data/config.txt`로 복사합니다.
7. [드워프 포트리스 한국어 번역 사전](https://github.com/refracta/df-translations)의 `translations` 디렉터리를 `dfint-data/translations`로 다운로드합니다.
8. [Noto CJK](https://github.com/notofonts/noto-cjk)를 다운로드하고 `NotoSansMonoCJKsc-Bold.otf`를 `dfint-data/fonts/NotoSansMonoCJKsc-Bold.otf`로 옮깁니다.
9. [dfint 구(舊) 사전](https://github.com/dfint/autobuild/blob/main/translation_build/csv/Korean/dfint_dictionary.csv)을 `dfint-data/legacy-dictionary.csv`로 다운로드합니다.

만약 설정이 어렵다면, [여기에서](https://github.com/refracta/dfint-rust-cjk-ko/releases) 최신 압축 패키지를 내려받아 필요한 파일을 그대로 꺼내서 사용하세요.
