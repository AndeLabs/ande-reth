# Ande-Reth - Fork Personalizado de ev-reth

**Fork de:** [evstack/ev-reth](https://github.com/evstack/ev-reth)
**Propósito:** Añadir precompile personalizado para Token Duality de ANDE
**Proyecto relacionado:** [andechain](../andechain)
**Estado Actual:** ✅ **Implementación Core Completa - Integración Pendiente (v0.2.0)**

---

## 🎯 ¿Por Qué Este Fork?

Este fork de ev-reth existe para implementar **Token Duality** en AndeChain, una característica que permite que el token ANDE funcione simultáneamente como:
1. Token nativo para pagar gas fees
2. Token ERC-20 estándar para dApps

### Modificación Principal

**Precompile Personalizado en dirección `0x00...fd`**:
- Permite a ANDEToken.sol ejecutar transferencias de balance nativo
- Elimina la necesidad de "Wrapped ANDE" (WANDE)
- Simplifica la experiencia de usuario

---

## 📂 Estructura del Proyecto

```
ev-reth/                          # Este repositorio (independiente)
├── crates/
│   ├── common/
│   ├── evolve/
│   ├── node/
│   └── tests/
├── bin/ev-reth/
├── Dockerfile                    # Construye imagen Docker personalizada
└── ANDE_README.md               # Este archivo

andechain/                        # Repositorio principal (separado)
├── contracts/
│   └── src/ANDEToken.sol        # Usa el precompile
├── infra/
│   └── docker-compose.yml       # Referencia imagen de este repo
├── docs/
│   └── token-duality/           # Documentación del feature
└── README.md
```

---

## 🔧 Modificaciones Realizadas

### 🎉 BREAKTHROUGH: Implementación Core Completa (v0.2.0)

**El precompile ahora EJECUTA transferencias de balance nativo reales** usando `journal.transfer()` de revm v29.

**✅ Lo que está implementado:**
- ✅ Transferencias de balance nativo reales via `JournalTr::transfer()`
- ✅ Validación de caller (solo ANDEToken en 0x00..fc puede llamar)
- ✅ Parsing de input (96 bytes: from, to, value)
- ✅ Cálculo de gas correcto
- ✅ Manejo completo de errores (balance insuficiente, errores de DB)
- ✅ Compila sin errores
- ✅ Validado contra implementación de producción de Celo

**⏳ Lo que falta (Integración):**
- ⏳ Registrar precompile con EVM de reth (requiere custom EvmConfig)
- ⏳ Tests unitarios para lógica del precompile
- ⏳ Tests de integración end-to-end con ANDEToken.sol
- ⏳ Build de imagen Docker v0.2.0

**Descubrimiento clave:** `PrecompileProvider<CTX>` con trait bound `CTX: ContextTr<Journal: JournalTr>` permite acceso al journal para modificaciones de balance durante ejecución del EVM.

**Ver:** `docs/V0.2.0_STATUS.md` para documentación completa del achievement.

---

### 1. Precompile ANDE Transfer (Stub - v0.1.0)

**Archivo a modificar:** (TBD - necesita investigación de reth-revm)

El precompile implementa:
```rust
// Dirección: 0x00000000000000000000000000000000000000fd
pub fn ande_native_transfer(
    input: &Bytes,
    gas_limit: u64,
    context: &mut EvmContext
) -> PrecompileResult {
    // Valida que caller == ANDEToken contract
    // Ejecuta transferencia de balance nativo
    from.balance -= value;
    to.balance += value;
}
```

### 2. Registro del Precompile

El precompile se registra en la configuración del EVM para que esté disponible en todas las transacciones.

---

## 🚀 Uso

### Para Desarrolladores de AndeChain

**No necesitas clonar este repo directamente**. El proyecto `andechain` usa la imagen Docker compilada.

```bash
# En andechain/infra/docker-compose.yml
services:
  ev-reth-sequencer:
    image: ghcr.io/ande-labs/ande-reth:token-duality-v1
```

### Para Contribuidores de Ande-Reth

Si necesitas modificar el precompile:

```bash
# 1. Clonar este repo
cd ~/dev/ande-labs
git clone https://github.com/ande-labs/ev-reth.git
cd ev-reth
git checkout feature/ande-precompile

# 2. Hacer cambios al precompile
# (Ver docs/token-duality/IMPLEMENTATION_PLAN.md en andechain)

# 3. Compilar
cargo build --release

# 4. Crear imagen Docker
docker build -t andelabs/ande-reth:local .

# 5. Probar en andechain
cd ../andechain/infra
# Modificar docker-compose.yml para usar andelabs/ande-reth:local
docker compose up -d
```

---

## 🔄 Sincronización con Upstream

### Mantener Actualizado

```bash
# Añadir upstream (solo una vez)
git remote add upstream https://github.com/evstack/ev-reth.git

# Actualizar desde upstream
git fetch upstream
git checkout main
git merge upstream/main

# Re-aplicar nuestros cambios
git checkout feature/ande-precompile
git rebase main
```

### Estrategia de Branching

```
main                    # Sincronizado con evstack/ev-reth
  └─ feature/ande-precompile   # Nuestras modificaciones
```

---

## 📦 Compilación

### Requisitos

- Rust 1.82+
- Docker (para imagen)
- ~10GB espacio en disco
- ~30-60 min primera compilación

### Compilar Binario

```bash
cargo build --release

# Binario resultante:
# target/release/ev-reth
```

### Compilar Imagen Docker

```bash
docker build -t andelabs/ande-reth:token-duality-v1 .

# Para publicar (requiere permisos):
docker tag andelabs/ande-reth:token-duality-v1 \
  ghcr.io/ande-labs/ande-reth:token-duality-v1

docker push ghcr.io/ande-labs/ande-reth:token-duality-v1
```

---

## 🧪 Testing

```bash
# Tests del proyecto
cargo test

# Tests específicos de precompile (cuando se implementen)
cargo test --package evolve-ev-reth --test precompile_tests
```

---

## 📚 Documentación Relacionada

### En Este Repo
- `README.md` - Documentación original de ev-reth
- `ANDE_README.md` - Este archivo
- `CHANGELOG.md` - Historial de cambios

### En Repo andechain
- `docs/token-duality/IMPLEMENTATION_PLAN.md` - Plan completo de Token Duality
- `docs/token-duality/QUICK_START.md` - Guía rápida
- `docs/ROADMAP_CONCEPTUAL.md` - Contexto del roadmap

---

## 🤝 Contribuir

### Para Issues/Bugs de Token Duality
Reportar en: [andechain/issues](https://github.com/ande-labs/andechain/issues)

### Para Issues de ev-reth Base
Considerar reportar en: [evstack/ev-reth/issues](https://github.com/evstack/ev-reth/issues)

### Pull Requests

1. Fork este repo
2. Crear branch: `git checkout -b feature/mi-mejora`
3. Commit: `git commit -m "feat(precompile): descripción"`
4. Push: `git push origin feature/mi-mejora`
5. Crear PR

---

## ⚠️ Notas Importantes

### Dependencias de reth

Este fork depende de `reth v1.7.0`. Actualizaciones de reth pueden requerir:
- Ajustar importaciones
- Adaptar interfaces
- Re-compilar completamente

### Compatibilidad

- ✅ Compatible con andechain v0.1.0+
- ✅ Funciona con genesis.json de AndeChain
- ⚠️ NO compatible con ev-reth upstream vanilla

### Seguridad

⚠️ **IMPORTANTE**: Este código incluye modificaciones al EVM.
- Requiere auditoría de seguridad antes de producción
- No usar en mainnet sin revisión profesional
- Bug bounty recomendado

---

## 📞 Contacto

- **Technical Lead:** dev@andelabs.io
- **Security:** security@andelabs.io
- **Discord:** https://discord.gg/andechain

---

## 📄 Licencia

Hereda la licencia dual de ev-reth:
- MIT License
- Apache License 2.0

Ver `LICENSE-MIT` y `LICENSE-APACHE` para detalles.

---

## 🔗 Enlaces Útiles

- **ev-reth upstream:** https://github.com/evstack/ev-reth
- **reth upstream:** https://github.com/paradigmxyz/reth
- **AndeChain:** https://github.com/ande-labs/andechain
- **Documentación de revm:** https://github.com/bluealloy/revm

---

**Última actualización:** 2025-10-07
**Versión:** 0.1.0-ande (basado en ev-reth main + precompile ANDE)
