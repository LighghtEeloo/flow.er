# Memo

## Model Draft (v0.3.0)
### Data Model for Storage

#### Key Concepts

1. Collection = HashMap + RelationModel
   - HashMap acts as a handy data bucket. All it cares is Id => Value.
   - RelationModel handles the relational info. Only Ids are operated so that it's fast and flexible.
     - `LinearModel`.
     - `FlowModel`: A tree-like model with ...
       - node: records descendants (children) and at most one elderly (parent).
       - root: maybe an Id of the root node.
       - orphans: no elderly and not root.

2. Id = Hash + Eq + TimeRep (+ Clone + Copy)
   - Ids should be able to yield from a given time (`TimeRep`).

#### Big Picture

- `Stockpile`
  - branch: `Branch`
    - cubes: `HashMap<CubeId, Cube>` where `Cube`:
      - name: `String`
      - id: `CubeId`
      - locked: `bool`
      - entries: `HashMap<EntryId, Entry>` where `Entry`:
        - ...
      - relation: `LinearModel<EntryId>`
    - flow: `FlowModel<CubeId>`
  - ...




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
