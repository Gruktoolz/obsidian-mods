#!/usr/bin/env python3
"""Genera index.json: los mods nativos que SÍ están compilados en el APK de Obsidian.
Fuente de verdad: herramientas/lote.json ("vivos") + metadatos del catálogo de Horizon.
Uso: python3 generar-indice.py   (luego git add index.json; commit; push)"""
import json, urllib.request, pathlib, datetime, hashlib, re, tomllib

# Cómo se maneja un mod, según los puntos del juego a los que se engancha (nombre sin «native_») → frase corta para la ficha.
USOS = [
    (("key", "input_key", "sort_press"), "Se maneja con teclas"),
    (("command", "commands", "perform_command", "register_commands"), "Tiene comandos de chat"),
    (("use_item", "use_item_on", "right_click_block"), "Se usa con clic derecho (ítem o bloque)"),
    (("screen_render_post", "screen_mouse_button_pressed_pre", "screen_mouse_button_released_pre", "panel_event", "menu_clicked",
      "draw_options", "options_mouse"), "Abre su propia pantalla (se maneja con toques)"),
    (("hud", "gui_layer_pre", "gui_layer_post", "health_hud", "minimap", "draw_chat", "draw_tooltip"), "Dibuja en pantalla (HUD)"),
    (("mods", "settings", "settings_click"), "Ajustes en Opciones › Mods"),
]


def interacciones(raiz, mid):
    """Qué hace falta para usar el mod: frases (de los enganches) y los comandos que registra con su texto literal."""
    d = raiz / "nativos" / mid / "fuentes" / mid
    toml = d / "horizon-mixin.toml"
    hooks = set()
    if toml.is_file():
        for x in tomllib.loads(toml.read_text(encoding="utf-8")).get("injection", []):
            hooks.add((x.get("hook") or x.get("target", "").split("::")[-1]).removeprefix("native_"))
    usos = [frase for nombres, frase in USOS if hooks & set(nombres)]
    comandos = []
    for f in sorted(d.rglob("*.rs")):
        for m in re.finditer(r'NativeCommand::(new|op)\(\s*"([^"]+)"', f.read_text(encoding="utf-8", errors="ignore")):
            uso = m.group(2)
            if uso not in [c["uso"] for c in comandos]:
                comandos.append({"uso": uso, "op": m.group(1) == "op"})
    return usos, comandos

raiz = pathlib.Path(__file__).parent
vivos = json.load(open(raiz / "herramientas/lote.json"))["vivos"]
compat = {x["id"]: x for x in json.load(open(raiz / "nativos/compatibilidad.json"))}
hz = json.load(urllib.request.urlopen("https://horizon-rust.github.io/horizon-mods/index.json"))
meta = {p["id"]: p for p in hz["projects"] if p.get("kind") == "native-mod"}
out = []
for i in sorted(vivos):
    p = meta.get(i)
    if not p or compat.get(i, {}).get("estado") != "encaja":
        continue
    v = p["versions"][0]
    # El archivo real que se sirve desde este repositorio (descargas/native-mod/<id>-<version>.hnmod)
    carpeta = raiz / "descargas/native-mod"
    f = next((c for n in (f"{i}-{v.get('version', '')}", f"{i.replace('_', '-')}-{v.get('version', '')}", i)
              if (c := carpeta / f"{n}.hnmod").is_file()), carpeta / "no-existe")
    if not f.is_file():
        print("SIN ARCHIVO, se omite:", i)
        continue
    data = f.read_bytes()
    # Teclas que usa el mod (de nativos/<id>/prueba.json): las muestra la ficha y alimentan el perfil de controles «Mods».
    teclas = []
    pj = raiz / "nativos" / i / "prueba.json"
    if pj.is_file():
        for e in json.load(open(pj)).get("entradas", []):
            if e.get("tipo") == "tecla" and e.get("tecla"):
                teclas.append({"tecla": e["tecla"].replace("key.keyboard.", ""), "accion": e.get("accion", ""),
                               "fija": "fija" in e.get("accion", "")})
    usos, comandos = interacciones(raiz, i)
    out.append({k: p[k] for k in ("id", "name", "summary", "description", "authors", "tags", "icon_url", "icon") if k in p}
               | {"version": v.get("version", ""), "teclas": teclas, "usos": usos, "comandos": comandos, "file": f.name, "size": len(data),
                  "sha256": hashlib.sha256(data).hexdigest()})
json.dump({"schema": 1, "generado": datetime.datetime.now(datetime.UTC).isoformat(timespec="seconds"), "nativos": out},
          open(raiz / "index.json", "w"), ensure_ascii=False, indent=1)
print(len(out), "nativos en index.json")
