# Windows: build i deploy (fork lokalny)

- **Katalog Cargo na dysku (`CARGO_TARGET_DIR` fizycznie)**: `C:\Users\Kuba\Pobrane\Moj_Handy\h`
- **Przy kompilacji release**: skrypt `deploy-handy.ps1` ustawia `subst W:` na ten folder i buduje z `CARGO_TARGET_DIR=W:\` (krótka ścieżka, bez folderu w korzeniu `C:\`).
- **Artefakty release**: `...\Moj_Handy\h\release\` (m.in. `handy.exe`, DLL, `resources\`)
- **Deploy do portable**: uruchom z katalogu repo:

```powershell
.\deploy-handy.ps1
```

Kopiuje z `Moj_Handy\h\release\` do `Moj_Handy\Handy\My_handy\`.

## Długa ścieżka (Whisper / MSVC)

Jeśli build pada na limitach ścieżki Windows, użyj domyślnego deployu (subst `W:` jest włączany automatycznie). Ręcznie:

```powershell
subst W: C:\Users\Kuba\Pobrane\Moj_Handy\h
$env:CARGO_TARGET_DIR = 'W:\'
bun tauri build
```

Nie twórz osobnych folderów build w korzeniu dysku C: — tylko `Moj_Handy\h` (+ opcjonalnie `subst W:`).

## Ikona pliku exe a kolor akcentu

- **Od razu (bez przebudowy):** okno aplikacji, pasek zadań (ikona okna) i zasobnik — kolor z ustawienia akcentu.
- **Ikona `handy.exe` w Eksploratorze / przypięty skrót:** zaszyta przy `bun tauri build`. Aby dopasować do akcentu:
  1. `.\scripts\regenerate_app_icon.ps1` (czyta `appearance_accent_color` z `settings_store.json`),
  2. przebuduj (`deploy-handy.ps1` sam przebuduje, gdy `icon.ico` jest nowszy niż `handy.exe`).
- **Skrót z własną ikoną:** po zmianie akcentu aplikacja zapisuje `handy-accent.ico` obok exe (lub przycisk w Ustawieniach → Wygląd).
