# Maneuver Format Guideline

Every maneuver in `rc-log` is maintained as a `.json` file inside `content/maneuvers/text/`. Each JSON file corresponds to exactly one maneuver. 

If there is a demonstration video for the maneuver, it should be placed in `content/maneuvers/videos/` sharing the **exact same filename** as the JSON (e.g., `helicopter_hover.webm` for `helicopter_hover.json`).

---

## JSON Schema

The maneuver file must follow this strict JSON format:

```json
{
    "vehicleType": "Plane",
    "name": "Maneuver Name",
    "description": "...",
    "difficulty": 1,
    "tags": [
        "Basic", 
        "Freestyle"
    ]
}
```

- **`vehicleType`**: Must be exactly `"Helicopter"`, `"Plane"`, or `"Drone"`.
- **`name`**: The display name of the maneuver (e.g., "Power Loop").
- **`difficulty`**: An integer from `1` (Beginner) to `7` (Extreme/Advanced).
- **`tags`**: An array of string tags categorizing the maneuver.
- **`description`**: A Markdown formatted string that acts as the core tutorial document.

---

## The Description Markdown Format

The `description` field is a parsed **Markdown text block**. When filling out the `description` string inside the JSON, you must adhere to the following standard RC-log tutorial structure:

### 1. Introduction
Start with a short, textual description providing context. What is the maneuver? What does it look like? Why learn it?

### 2. Step-by-Step Guide
Provide a concise, numbered list explaining the mechanical input execution.
*Use `### Execution Steps` as the markdown header.*

### 3. Troubleshooting & Common Problems
Finish with a section detailing hints to fight common problems. What usually goes wrong for beginners? How can they fix it?
*Use `### Common Problems` or `### Pro Tips` as the markdown header.*

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
