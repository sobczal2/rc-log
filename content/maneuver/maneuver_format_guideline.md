# Maneuver Format Guideline

Every maneuver in `rc-log` lives in its own subdirectory under `content/maneuvers/`, named using snake_case (e.g., `helicopter_hover/`). Each maneuver directory contains a `maneuver.json` plus one or more variation subdirectories.

---

## Directory Layout

```
content/maneuvers/
  helicopter_hover/
    maneuver.json           # maneuver metadata and overview description
    default/
      variation.json        # name + execution description for this variation
      video.mp4             # optional demonstration video
    inverted/               # optional additional variation
      variation.json
      video.mp4
```

- Every maneuver must have exactly one `default/` variation subdirectory.
- Additional variation directories can be added alongside `default/` using any snake_case name.
- A video file inside a variation directory is optional. When present, it must be named `video.<ext>` (e.g., `video.mp4`).

---

## maneuver.json Schema

```json
{
  "vehicleType": "Helicopter",
  "name": "Hover",
  "difficulty": 1,
  "tags": ["Basic", "Fundamentals"],
  "description": "Short introductory paragraph describing what the maneuver is and why it matters."
}
```

- **`vehicleType`**: Must be exactly `"Helicopter"`, `"Plane"`, or `"Drone"`.
- **`name`**: The display name of the maneuver (e.g., "4-Point Hover").
- **`difficulty`**: An integer from `1` (Beginner) to `7` (Extreme/Advanced).
- **`tags`**: An array of string tags categorizing the maneuver.
- **`description`**: A short Markdown introduction. No execution steps here — those belong in each variation's `variation.json`.

---

## variation.json Schema

```json
{
  "name": "Default",
  "description": "### Execution Steps\n1. ...\n\n### Common Problems\n- **Problem:** ..."
}
```

- **`name`**: Short display name for this variation (e.g., `"Default"`, `"Inverted"`, `"Fast Pirouette"`).
- **`description`**: Full Markdown tutorial for this specific execution style. Must follow the structure below.

---

## Variation Description Markdown Format

The `description` field inside `variation.json` is a Markdown text block. Adhere to the following structure:

### 1. Step-by-Step Guide
Concise numbered list of mechanical input steps for this variation.
*Use `### Execution Steps` as the header.*

### 2. Troubleshooting & Common Problems
Hints for common mistakes specific to this variation.
*Use `### Common Problems` or `### Pro Tips` as the header.*

---

### Example Description Block

```markdown
A Power Loop is a freestyle FPV standard where the quadcopter performs a huge, backwards looping arc over an obstacle, maintaining visual contact throughout.

### Execution Steps
1. Approach the target carrying substantial forward speed.
2. Pitch backwards sharply while increasing the throttle to launch smoothly over the obstacle.
3. As the quad reaches the apex (inverted), drop the throttle immediately.
4. Let your momentum carry you over, pitch back to level, and catch yourself smoothly.

### Common Problems
- **Losing Altitude Too Fast**: You are dropping your throttle too early before hitting the true apex of the loop.
- **Crashing into the Obstacle**: You aren't pitching back hard enough on entry. Be aggressive with the initial pitch up while blipping the throttle to give yourself upward momentum.
```
