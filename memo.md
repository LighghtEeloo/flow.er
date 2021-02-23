# Memo

## Next Step

Some "next generation" feature requests.

1. Name: flow-er
2. Entity-based: nodes & tags.
3. Model can be viewed in 4 different ways: 
   1. LinearList
   2. BranchFlow
   3. TimeShift
   4. Calendar
4. related keyword prompt
5. better UI
6. CLI style integration
7. advanced data operation (copy / move)
8. Safer file / cloud storage
9. Dry export

## Todo List

- [ ] Router
- [ ] vessel.vm (Dancer + Animator)
- [ ] Architect for flow
- [ ] Critic

## Vessel

### Soul

1. Architect
    1. add
    2. check
    3. link
    4. del
2. Animator
    1. compute
    2. illustrate
3. Dancer
    1. current

    2. focus

    3. wander

### Blood

### Bone





----



## Old Memo Select

### Model Draft (v0.2.1)

**Note: This model is no longer representitive after v0.2.2.**

**Key Concepts**

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

**Big Picture**

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



### Todo List

- [ ] Router.
  - [x] Router buttons.
  - [x] Src view.
  - [x] History.
  - [ ] Settings.
- [ ] Better FlowModel -> Branch.
  - [ ] How to manage / show cubes.
  - [ ] How to present.
  - [ ] How to operate on UI.
- [ ] Cube Management
  - [x] Generalized Id System.
  - [x] Cube Id & searching.
  - [ ] Cube RelationModel.
    - [x] General.
    - [ ] Wander.
    - [ ] Link.
  - [x] Cube -> Stockpile/Branch.
  - [x] Cube storage.
  - [x] Separate Cube Msg.
  - [ ] ~~Use &mut Cube instead of clone().~~
- [ ] Better Node
  - [x] Node CSS.
  - [ ] Move node with keyboard.
  - [ ] Bubble details.
- [ ] Filter.
- [ ] Tag.
- [ ] Caret-position-based new line.


### Fix List
- [x] Better model which preserves the first node (title does the job).
- [x] Queue-preserving deletion.
- [x] Delayed deletion (with safe-lock).
- [ ] No double backspace && no force delete.
- [x] New cube as root / orphan. What to show on startup? 
- [x] NodeRef should be updated if node altered.
- [ ] Refresh CubeModel if changed outside.



### Done List
- [x] Support clipboard exporting / importing.
  - [x] Support json.
  - [x] Interface.
  - [x] Msg Update.
  - [x] Ctrl + /.
  - [x] Automatic.

