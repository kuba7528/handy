# Windows: build i deploy (fork lokalny)

- **Domyślny katalog Cargo (`CARGO_TARGET_DIR`)**: `C:\Users\Kuba\Pobrane\Moj_Handy\h` (cache)
- **Zalecany przy przebudowie** (krótsza ścieżka, mniej problemów z linkowaniem whisper/MSVC): `C:\ht`
- **Artefakty release**: `...\Moj_Handy\h\release\` (m.in. `handy.exe`, DLL, `resources\`)
- **Deploy do portable**: uruchom z katalogu repo:

```powershell
.\deploy-handy.ps1 -TargetDir C:\ht
```

Kopiuje z `C:\ht\release\` (lub `Moj_Handy\h\release\`) do `Moj_Handy\Handy\My_handy\`.

## Długa ścieżka (Whisper / MSVC)

Jeśli build pada na limitach ścieżki Windows, użyj krótszego katalogu **w Pobrane** (nie `C:\ht` w korzeniu dysku):

```powershell
.\deploy-handy.ps1 -TargetDir "C:\Users\Kuba\Pobrane\Moj_Handy\ht"
```

Nadal unikamy `C:\ht` na dysku C:.

## Ikona pliku exe a kolor akcentu

- **Od razu (bez przebudowy):** okno aplikacji, pasek zadań (ikona okna) i zasobnik — kolor z ustawienia akcentu.
- **Ikona `handy.exe` w Eksploratorze / przypięty skrót:** zaszyta przy `bun tauri build`. Aby dopasować do akcentu:
  1. `.\scripts\regenerate_app_icon.ps1` (czyta `appearance_accent_color` z `settings_store.json`),
  2. przebuduj (`deploy-handy.ps1` sam przebuduje, gdy `icon.ico` jest nowszy niż `handy.exe`).
- **Skrót z własną ikoną:** po zmianie akcentu aplikacja zapisuje `handy-accent.ico` obok exe (lub przycisk w Ustawieniach → Wygląd).
