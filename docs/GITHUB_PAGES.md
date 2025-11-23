# 🌐 Despliegue en GitHub Pages

## 📋 Requisitos Previos

- Cuenta de GitHub
- Repositorio NanoCalc en GitHub
- Git instalado localmente

## 🚀 Configuración Inicial

### 1. Habilitar GitHub Pages

1. Ve a tu repositorio en GitHub
2. Click en **Settings** (Configuración)
3. En el menú lateral, click en **Pages**
4. En **Source**, selecciona:
   - **Deploy from a branch**
   - Branch: `main`
   - Folder: `/docs`
5. Click en **Save**

### 2. Estructura de Archivos

```
NanoCalc/
├── docs/
│   ├── index.html          # Landing page principal
│   └── app/
│       ├── index.html      # Web app
│       ├── nanocalc.js     # WASM bindings (generado)
│       └── nanocalc_bg.wasm # WASM binary (generado)
├── .github/
│   └── workflows/
│       └── deploy.yml      # GitHub Actions
└── Trunk.toml              # Configuración de Trunk
```

## 🔨 Compilación Manual

### Compilar versión WASM

```bash
# 1. Instalar herramientas necesarias
cargo install trunk
cargo install wasm-bindgen-cli

# 2. Agregar target WASM
rustup target add wasm32-unknown-unknown

# 3. Compilar para web
trunk build --release --public-url /NanoCalc/app/

# 4. Copiar archivos a docs
mkdir -p docs/app
cp -r dist/* docs/app/

# 5. Commit y push
git add docs/
git commit -m "Update web build"
git push origin main
```

## ⚙️ Despliegue Automático con GitHub Actions

El archivo `.github/workflows/deploy.yml` automatiza el proceso:

1. **Trigger**: Se ejecuta en cada push a `main`
2. **Instala**: Rust, wasm-bindgen, trunk
3. **Compila**: Genera archivos WASM
4. **Despliega**: Sube a GitHub Pages

### Verificar el despliegue

1. Ve a **Actions** en tu repositorio
2. Verifica que el workflow se ejecutó exitosamente
3. Accede a: `https://tuusuario.github.io/NanoCalc/`

## 🌍 URLs de Acceso

Después del despliegue, tu app estará disponible en:

- **Landing Page**: `https://tuusuario.github.io/NanoCalc/`
- **Web App**: `https://tuusuario.github.io/NanoCalc/app/`

## 🎨 Personalización de la Landing Page

### Editar información

Abre `docs/index.html` y modifica:

```html
<!-- Información del desarrollador -->
<div>
    <h3>About</h3>
    <p>Developed by <strong>Tu Nombre</strong></p>
</div>

<!-- Links de redes sociales -->
<a href="https://github.com/tuusuario">
    <i class="fab fa-github"></i>
</a>
```

### Cambiar colores

```css
:root {
    --primary: #667eea;    /* Color primario */
    --secondary: #764ba2;  /* Color secundario */
    --accent: #f093fb;     /* Color de acento */
}
```

## 📦 Releases y Descargas

### Crear un Release

```bash
# 1. Crear tag
git tag -a v0.1.0 -m "First release"
git push origin v0.1.0

# 2. Compilar binarios para cada plataforma

# Windows
cargo build --release --target x86_64-pc-windows-msvc

# macOS
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
lipo -create target/x86_64-apple-darwin/release/nanocalc \
             target/aarch64-apple-darwin/release/nanocalc \
             -output nanocalc-macos-universal

# Linux
cargo build --release --target x86_64-unknown-linux-gnu
```

### Subir binarios

1. Ve a **Releases** en GitHub
2. Click en **Draft a new release**
3. Selecciona el tag `v0.1.0`
4. Sube los binarios:
   - `nanocalc-windows-x64.exe`
   - `nanocalc-macos-universal.dmg`
   - `nanocalc-linux-x64.AppImage`
5. Click en **Publish release**

## 🔍 Debugging

### La página no carga

1. Verifica que GitHub Pages está habilitado
2. Asegúrate de que los archivos están en `/docs`
3. Espera ~5 minutos para propagación

### Error 404

- Verifica que `public-url` en Trunk.toml sea correcto:
  ```toml
  [build]
  target = "index.html"
  public-url = "/NanoCalc/app/"
  ```

### WASM no carga

1. Abre la consola del navegador (F12)
2. Verifica errores de CORS o carga
3. Asegúrate de que los archivos `.wasm` y `.js` existen en `docs/app/`

## 📊 Analítica (Opcional)

### Agregar Google Analytics

```html
<!-- En docs/index.html, antes de </head> -->
<script async src="https://www.googletagmanager.com/gtag/js?id=GA_MEASUREMENT_ID"></script>
<script>
  window.dataLayer = window.dataLayer || [];
  function gtag(){dataLayer.push(arguments);}
  gtag('js', new Date());
  gtag('config', 'GA_MEASUREMENT_ID');
</script>
```

## 🎯 Checklist Final

- [ ] Repositorio en GitHub creado
- [ ] GitHub Pages habilitado desde `/docs`
- [ ] Landing page (`docs/index.html`) personalizada
- [ ] WASM compilado y copiado a `docs/app/`
- [ ] GitHub Actions configurado (`.github/workflows/deploy.yml`)
- [ ] Primer commit y push realizados
- [ ] Release v0.1.0 creado con binarios
- [ ] URLs verificadas y funcionando
- [ ] README actualizado con links

## 🆘 Soporte

Si tienes problemas:

1. Verifica los logs de GitHub Actions
2. Revisa la consola del navegador
3. Abre un issue en el repositorio
4. Consulta la documentación de GitHub Pages

---

**¡Tu aplicación NanoCalc está lista para el mundo! 🚀**