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
  - [ ] Ctrl + /.
  - [x] Automatic.
- [ ] Cube Management
  - [x] Generalized Id System.
  - [x] Cube Id & searching.
  - [ ] Cube RelationModel.
    - [x] General.
    - [ ] Wander.
  - [x] Cube -> Stockpile/Branch.
  - [x] Cube storage.
  - [x] Separate Cube Msg.
  - [ ] Use &mut Cube instead of clone().
- [ ] Better Node
  - [x] Node CSS.
  - [ ] Move node with keyboard.
  - [ ] Bubble details.
- [ ] Filter.
- [ ] Caret-position-based new line.


## Fix List
- [x] Better model which preserves the first node (title does the job).
- [x] Queue-preserving deletion.
- [x] Delayed deletion (with safe-lock).
- [ ] No double backspace && no force delete.
