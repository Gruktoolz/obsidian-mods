#!/usr/bin/env python3
"""Genera index.json: los mods nativos que SÍ están compilados en el APK de Obsidian.
Fuente de verdad: herramientas/lote.json ("vivos") + metadatos del catálogo de Horizon.
Uso: python3 generar-indice.py   (luego git add index.json; commit; push)"""
import json, urllib.request, pathlib, datetime, hashlib
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
    out.append({k: p[k] for k in ("id", "name", "summary", "description", "authors", "tags", "icon_url", "icon") if k in p}
               | {"version": v.get("version", ""), "teclas": teclas, "file": f.name, "size": len(data),
                  "sha256": hashlib.sha256(data).hexdigest()})
json.dump({"schema": 1, "generado": datetime.datetime.now(datetime.UTC).isoformat(timespec="seconds"), "nativos": out},
          open(raiz / "index.json", "w"), ensure_ascii=False, indent=1)
print(len(out), "nativos en index.json")
