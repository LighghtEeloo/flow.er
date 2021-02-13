# Memo

## Relation-Model trait
- store message with position
  - type_str
  - add (pos)
  - del (pos)
- record position (focus) in the network
  - data - positional ref
- move around in the network
  - shift-enter: deepin & surface
  - up/down: wander
    - shift: fix

## Todo List
- [ ] Support clipboard exporting / importing.
  - [x] Support json.
  - [x] Interface.
  - [x] Msg Update.
  - [ ] Automatic.
- [ ] Support cube management.
  - [x] Generalized Id System.
  - [ ] Cube Id & searching.
  - [ ] Cube RelationModel.
  - [ ] Cube -> Stockpile/Branch.
  - [ ] Cube storage.
  - [x] Separate Cube Msg.
- [x] Node CSS.
- [ ] Bubble details.
- [ ] Filter.


## Fix List
- [x] Better model which preserves the first node (title does the job).
- [ ] Delayed deletion (with safe-lock).
- [ ] Caret-position-based new line.
- [ ] Queue-preserving deletion.
