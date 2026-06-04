# Windows: build i deploy (fork lokalny)

- **Domyślny katalog Cargo (`CARGO_TARGET_DIR`)**: `C:\Users\Kuba\Pobrane\Moj_Handy\h`
- **Artefakty release**: `...\Moj_Handy\h\release\` (m.in. `handy.exe`, DLL, `resources\`)
- **Deploy do portable**: uruchom z katalogu repo:

```powershell
.\deploy-handy.ps1
```

Kopiuje z `Moj_Handy\h\release\` do `Moj_Handy\Handy\My_handy\`.

## Długa ścieżka (Whisper / MSVC)

Jeśli build pada na limitach ścieżki Windows, użyj krótszego katalogu **w Pobrane** (nie `C:\ht` w korzeniu dysku):

```powershell
.\deploy-handy.ps1 -TargetDir "C:\Users\Kuba\Pobrane\Moj_Handy\ht"
```

Nadal unikamy `C:\ht` na dysku C:.
