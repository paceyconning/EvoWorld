# EvoWorld Development Roadmap

*This roadmap is automatically updated whenever [CONTEXT.md](../CONTEXT.md) is updated, ensuring it always reflects the latest project progress and priorities.*

---

## 📅 Phase 1: Immediate Priorities (Critical Path)

- [x] **Implement missing TerrainGenerator**  
  _Backend: ✅ Complete procedural terrain generation with biomes, rivers, minerals, and structures_
- [x] **Create basic Godot scenes**  
  _Frontend: Humanoid.tscn, Resource.tscn, Building.tscn_
- [x] **Set up database schema**  
  _Backend: Ensure DB operations work_
- [x] **Fix all compilation errors**  
  _Backend: ✅ All 22 compilation errors resolved - project now compiles successfully_
- [ ] **Complete AI behavior trees**  
  _Backend: Implement decision-making logic for humanoids and tribes_
- [ ] **Complete core simulation loop**  
  _Backend: End-to-end run, WebSocket, persistence_
- [ ] **Minimal frontend rendering**  
  _Frontend: 3D world, camera, UI_

---

## 🖥️ Backend Server Milestones (Prioritized Before Further Frontend Work)

- [x] **Fix compilation issues:** ✅ Resolved WebSocket imports, borrowing conflicts, missing imports, sqlx queries, recursive async functions, and moved value issues
- [x] **Complete terrain generation:** ✅ Multi-scale noise generation, realistic biomes, river systems, mineral deposits, and terrain structures
- [~] **Enhance simulation engine:** Optimize tick/update logic, add logging, improve resource/event processing
- [ ] **Expand AI behavior:** Add learning, adaptation, more actions, richer personality/memory
- [ ] **Deepen social & cultural systems:** Cultural transmission, conflict, alliances, social events
- [ ] **Improve environmental & resource systems:** Ecosystem dynamics, environmental impact modeling
- [ ] **Expand analytics engine:** Richer evolution metrics, detailed event/population tracking
- [ ] **Optimize database & persistence:** Schema, queries, auto-save, backup, recovery
- [ ] **Enhance WebSocket/server:** Real-time streaming, batching, filtering, subscriptions, connection management

---

## 🚀 Phase 2: Short-term Goals

- [ ] **Complete AI behavior trees**  
  _Backend: Implement decision-making logic for humanoids and tribes_
- [ ] **Re-enable database integration**  
  _Backend: Fix sqlx queries with proper DATABASE_URL configuration_
- [ ] **Enhance resource management**  
  _Backend: Full resource spawning, consumption, and regeneration_
- [ ] **Expand social systems**  
  _Backend: Tribe/culture, conflict, transmission_
- [ ] **Improve visualization**  
  _Frontend: Terrain, animation, UI/UX_

---

## 🌟 Phase 3+: Long-term & Advanced Features

- [ ] **Modding support**
- [ ] **Advanced analytics**
- [ ] **Performance optimization**
- [ ] **Sound design, advanced graphics**
- [ ] **User experience improvements**

---

## 🔄 How this roadmap is maintained
- This file is automatically updated in sync with [CONTEXT.md](../CONTEXT.md) to reflect all major project changes, progress, and new priorities.