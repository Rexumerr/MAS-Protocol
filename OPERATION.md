# 🕹️ Manual de Operaciones: Multiversal Agentic Swarm

Este documento detalla cómo operar tu infraestructura de élite con el mínimo esfuerzo.

## 🛡️ Seguridad Neural (Neural Gate)

Hemos elevado los estándares. El sistema ahora requiere una llave de acceso para aceptar datos.

1.  **Configura tu Token:** Ve a GitHub -> Settings -> Secrets -> Actions y añade un secreto llamado `ORACLE_TOKEN`. Ponle una frase compleja.
2.  **Operación Local:** Si ejecutas scripts localmente, asegúrate de exportar la variable:
    ```bash
    export ORACLE_TOKEN="tu_clave_secreta"
    npm run sync
    ```

## 🔐 Privacidad Local-First

El archivo `multiverse-state.json` ahora está **ofuscado** en Base64. Esto evita que la competencia o bots puedan leer tus niveles de negocio simplemente inspeccionando el código fuente. El dashboard de Astro se encarga de descifrarlo automáticamente durante la construcción.

## 📈 La Estrategia del 99 (Roadmap de Dominio)

1.  **Fase 1: Grind de Datos.** Usa agentes para extraer información de mercado. Esto subirá tu nivel de `DataMining`.
2.  **Fase 2: Automatización de Flujos.** Implementa bots de respuesta rápida. Desbloquea la capacidad de manejar más de 10 clientes en paralelo.
3.  **Fase 3: High Ticket Quests.** Activa la quest "Regional Market Dominance" una vez que tu `Infrastructure` sea nivel 70+.

## 🛠️ Mantenimiento Zero-Touch

- **Auto-Build:** Cada commit en GitHub dispara una compilación de Rust a WASM y despliega la web.
- **Auto-Heal:** El cerebro detecta si un agente falla y lo reinicia automáticamente en el próximo ciclo de procesamiento.

---

**Recuerda:** En el mercado de High Ticket, no eres un programador, eres el **Arquitecto del Multiverso**. Úsalo para demostrar poder.
