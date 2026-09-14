states:

- Counting (deadline)
  - on app update:
    - redraw (prints updated seconds remaining)
    - if deadline reached, play sound and switch to completed state
  - controls:
    - pause (go to paused state)
    - skip (start Counting with next countdown)
- Paused
  - pulse background relative to absolute time
  - controls:
    - resume (go back to Counting state)
    - skip (start Counting with next countdown)
- Completed
  - store time of previous ping
  - if previous ping was >10s ago, ping again and set updated time
  - controls:
    - resume (go to Counting state for next countdown)