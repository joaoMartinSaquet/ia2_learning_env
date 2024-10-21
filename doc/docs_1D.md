
# Games states transition machine
The game operates within four primary states:

1. **Started**:
   - This is the initial state where the game is set up and awaiting user input to start running.

2. **Running**:
   - In this state, the game is actively processing its logic, managing events, and updating the game world.

3. **Paused**:
   - The game's logic is temporarily halted in this state. No updates are processed until the game is resumed.

4. **Ended**:
   - This final state occurs when the game has concluded, and the player's score is calculated and displayed.

These states dictate how the game flows, with transitions occurring based on user inputs or game events.

There are four key events that trigger state transitions:

- **KeyS pressed (`s`)**: Starts or resumes the game.
- **KeyE pressed (`e`)**: Ends the game when pressed.
- **End of a timer (`end_timer`)**: Automatically ends the game after a specified duration (X seconds).
- **KeyR pressed (`r`)**: Resets the game, returning it to the initial state.

```mermaid
stateDiagram-v2
    Started --> Running : s
    Running --> Paused : s
    Paused --> Running : s
    Paused --> Ended : e 
    Running --> Ended : end_timer
    Ended --> Started : r
```
# environment Control

TBD yet #TODO
the environment is controlled by a controller (mouse) movement and take in input delta_x wich move the player as $x_{t+1} = dx + x_t$
```

        ┌─────────────┐  ball trajectory
        │             ├───────►         
        │             │                 
dx      │             │                 
 ──────►│             │                 
        │ environment │                 
        │             │                 
        │             │  player distance
        │             ├───────►         
        │             │                 
        └─────────────┘                 
```

# Trajectory 
## 1. deterministic trajectory
the ball folow a straight line with an initial direction going rightward, the direction goes to rightward to leftward when the ball reach the window's right bounds, the direction change every times the ball reach a window's bounds

# Rewards / Score

## 1. gaussian
The score is calculated using a Gaussian distribution, as defined by the following formula:

$$
\text{Score} = e^{-\frac{(x_f - x_p)}{\text{STD\_SCORE}}}
$$

Where:
- \( x_f \) is the final value,
- \( x_p \) is the predicted value,
- \(\text{STD\_SCORE}\) represents the standard deviation.

The maximum possible score is calculated as:

$$
\text{Maximum Score} = 0.01 \times 10 \, \text{seconds} = 1000.0
$$

<div style="text-align: center;">
    <img src="images/score_images/Gaussian_score.png" alt="Gaussian score" />
</div>

## 2. square 

The score is calculated using a square signal of the distance, if the distance is less or equal to PIXEL_DIST_METRIC then it is equals to 1.0 either way it's equals to 0.0

The maximum possible score is calculated as:

$$
\text{Maximum Score} = 0.01 \times 10 \, \text{seconds} = 1000.0
$$

<div style="text-align: center;">
    <img src="images/score_images/square_dist_metric.png" alt="Square score" />
</div>

