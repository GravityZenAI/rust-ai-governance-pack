# verify-agent-work.ps1
# Mini-CI para verificar que un agente completó su trabajo.
# Uso: .\verify-agent-work.ps1 -NotesFile "AGENT_NOTES.md"
#
# QUE HACE:
#   1. Lee el archivo de notas del agente
#   2. Cuenta checkboxes totales vs completados
#   3. Verifica que "COMPLETADO" exista al final
#   4. Revisa si los archivos mencionados fueron realmente modificados
#   5. Da un veredicto: PASS o FAIL con razones
#
# CUANDO SE USA:
#   - Despues de que un agente diga "termine"
#   - Moises o el asistente principal lo corre para verificar

param(
    [string]$NotesFile = "AGENT_NOTES.md",
    [string]$SkillsDir = ".agent\skills"
)

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  VERIFICADOR DE TRABAJO DE AGENTE" -ForegroundColor Cyan
Write-Host "  Mini-CI para colmenas y skills" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

$errors = @()
$warnings = @()
$passes = @()

# --- CHECK 1: El archivo de notas existe ---
if (-not (Test-Path $NotesFile)) {
    Write-Host "[FAIL] El archivo $NotesFile no existe." -ForegroundColor Red
    Write-Host "El agente ni siquiera creo el archivo de comunicacion."
    exit 1
}
$passes += "Archivo de notas existe"

# --- CHECK 2: Leer contenido ---
$content = Get-Content $NotesFile -Raw
$lines = Get-Content $NotesFile

# --- CHECK 3: Buscar COMPLETADO ---
if ($content -match "COMPLETADO") {
    $passes += "Marca COMPLETADO encontrada"
} else {
    $errors += "No se encontro 'COMPLETADO' en el archivo. El agente no declaro que termino."
}

# --- CHECK 4: Contar checkboxes ---
$totalBoxes = ($lines | Select-String -Pattern "\[[ x/]\]" -AllMatches).Matches.Count
$checkedBoxes = ($lines | Select-String -Pattern "\[x\]" -AllMatches).Matches.Count
$uncheckedBoxes = ($lines | Select-String -Pattern "\[ \]" -AllMatches).Matches.Count
$inProgressBoxes = ($lines | Select-String -Pattern "\[/\]" -AllMatches).Matches.Count

Write-Host "--- Checkboxes ---" -ForegroundColor Yellow
Write-Host "  Total:        $totalBoxes"
Write-Host "  Completados:  $checkedBoxes" -ForegroundColor Green
Write-Host "  Pendientes:   $uncheckedBoxes" -ForegroundColor Red
Write-Host "  En progreso:  $inProgressBoxes" -ForegroundColor Yellow
Write-Host ""

if ($uncheckedBoxes -gt 0) {
    $errors += "$uncheckedBoxes checkboxes sin completar"
    # Mostrar cuales faltan
    $lineNum = 0
    foreach ($line in $lines) {
        $lineNum++
        if ($line -match "\[ \]") {
            Write-Host "  Linea $lineNum : $($line.Trim())" -ForegroundColor Red
        }
    }
} else {
    $passes += "Todos los checkboxes completados ($checkedBoxes/$totalBoxes)"
}

if ($inProgressBoxes -gt 0) {
    $warnings += "$inProgressBoxes checkboxes aun en progreso"
}

# --- CHECK 5: Verificar que las skills fueron modificadas ---
if (Test-Path $SkillsDir) {
    Write-Host ""
    Write-Host "--- Skills modificadas ---" -ForegroundColor Yellow
    $skills = Get-ChildItem $SkillsDir -Directory
    $recentlyModified = 0
    $today = (Get-Date).Date

    foreach ($skill in $skills) {
        $skillFile = Join-Path $skill.FullName "SKILL.md"
        if (Test-Path $skillFile) {
            $lastWrite = (Get-Item $skillFile).LastWriteTime
            $isRecent = $lastWrite.Date -eq $today
            if ($isRecent) {
                $recentlyModified++
                Write-Host "  [MOD] $($skill.Name) - modificado $($lastWrite.ToString('HH:mm'))" -ForegroundColor Green
            } else {
                Write-Host "  [---] $($skill.Name) - sin cambios recientes" -ForegroundColor Gray
            }
        }
    }

    Write-Host ""
    Write-Host "  Skills modificadas hoy: $recentlyModified / $($skills.Count)"

    if ($recentlyModified -eq 0) {
        $errors += "Ninguna skill fue modificada. El agente dijo que termino pero no toco ningun archivo."
    } elseif ($recentlyModified -lt $skills.Count) {
        $warnings += "Solo $recentlyModified de $($skills.Count) skills fueron modificadas"
    } else {
        $passes += "Todas las skills ($($skills.Count)) fueron modificadas"
    }
}

# --- CHECK 6: Buscar preguntas sin responder ---
$preguntasSection = $false
$preguntasPendientes = 0
foreach ($line in $lines) {
    if ($line -match "Preguntas") { $preguntasSection = $true }
    if ($preguntasSection -and $line -match "^\s*-\s+\?" ) { $preguntasPendientes++ }
}
if ($preguntasPendientes -gt 0) {
    $warnings += "$preguntasPendientes preguntas pendientes del agente"
}

# --- VEREDICTO FINAL ---
Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  VEREDICTO" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

foreach ($p in $passes) {
    Write-Host "  [PASS] $p" -ForegroundColor Green
}
foreach ($w in $warnings) {
    Write-Host "  [WARN] $w" -ForegroundColor Yellow
}
foreach ($e in $errors) {
    Write-Host "  [FAIL] $e" -ForegroundColor Red
}

Write-Host ""
if ($errors.Count -eq 0) {
    Write-Host "  === RESULTADO: PASS ===" -ForegroundColor Green
    Write-Host "  El agente completo su trabajo correctamente." -ForegroundColor Green
    exit 0
} else {
    Write-Host "  === RESULTADO: FAIL ===" -ForegroundColor Red
    Write-Host "  El agente tiene $($errors.Count) error(es). Trabajo incompleto." -ForegroundColor Red
    exit 1
}
