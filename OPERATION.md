# 🕹️ Manual de Operaciones: Rexumer MAS-Protocol

Este documento detalla la operativa de tu infraestructura de élite bajo el modelo **Edge-First**.

## 🛡️ Neural Gate & Privacidad
El sistema opera bajo un esquema de **Soberanía Digital**. Todo dato está ofuscado y protegido por un token de acceso único.

1.  **Identity:** El sistema está bajo la firma de **Rexumer**. 
2.  **Auth:** Requiere `ORACLE_TOKEN` en GitHub Secrets para despliegues.

## 📱 Operativa Edge-First (Mobile)
A diferencia de los modelos cloud tradicionales, el MAS-Protocol prioriza la ejecución local:

1.  **Local Sync:** Usa `npm run sync` en Termux para procesar datos antes del push.
2.  **APK Deployment:** El dashboard vive en tu dispositivo. Una vez generado el build, usa `npx cap sync android` para actualizar tu APK sin pasar por la web pública.

## 📈 Roadmap de Dominio Agentico
1.  **Fase 1:** Recolección local de métricas.
2.  **Fase 2:** Inferencia del Ultra-Brain en el dispositivo.
3.  **Fase 3:** Autonomía total sin dependencia de la nube de GitHub.

---
*Rexumer Strategic Operations - 2026*
