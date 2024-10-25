# To-Do List

## ✏️ Work Tasks
- [x] Compute a score of ball follow
- [x] Display the score
- [x] Mouse control input
- [x] Add random trajectory
- [ ] Add a new trajectory
- [x] Complete Readme to describe how app works plus trajectory implemented
- [x] fixed y dimension in mouse control
- [x] Timer
- [x] write res in a file to plot it and do the plotter
- [x] reset the game
- [x] fix score seems too big
- [x] fix state machine
- [x] tune the score metrics, maybe change it 
- [ ] control the player using an another softwares ( décision from a python that send commands to the game) https://arrow.apache.org
- [x] player can displace outside the frame, fix it
- [ ] choixe of strategy with alpha num
- [x] the moves in random strategy are too sudden fix it
- [x] save score in a file see LastMouseDisp in ressources
- [x] do the doc of log parts
- [ ] add replay the episodes from log files
- [ ] try to do a learning based on a log files ( basic DNN or even CGP, or be crazy GRN ?!  )
- [ ] Replay episodes with action coming from a file
- [ ] add a idication before the ball change the directions ?!
- [ ] add other input device (input from a file ) see notes below 
- [ ] add change of trajectory in game
- [ ] with 0mq (https://zeromq.org)
- [ ] ebnd file reading command
## 📝 Notes

- the score metrics is quite strange, it comes surely from the timesteps too many call ( Maybe need to scale it to one ! max score is 1000)
- Add a state machine for input controller, this input should be possible only if the game is in start mode !


