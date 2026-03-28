# Maneuver Format Guideline

Every maneuver in `rc-log` lives in its own subdirectory under `content/maneuver/`, named using snake_case (e.g., `helicopter_hover/`). Each maneuver directory contains a `maneuver.json` plus one or more variation subdirectories.

---

## Directory Layout

```
content/maneuver/
  helicopter_hover/
    maneuver.json           # maneuver metadata and full description
    default/
      variation.json        # variation-specific metadata and description
      video.mp4             # optional demonstration video
    nose_in/                # optional additional variation
      variation.json
      video.mp4
```

- Every maneuver must have exactly one `default/` variation subdirectory.
- Additional variation directories can be added alongside `default/` using any snake_case name.
- Every variation directory must contain a `variation.json`.
- A video file is optional. When present, it must be named `video.<ext>` (e.g., `video.mp4`).

---

## maneuver.json Schema

```json
{
  "id": "a56b9ef1-41fd-432d-aeba-380214f1f669",
  "vehicleType": "Helicopter",
  "name": "Hover",
  "difficulty": 1,
  "tags": ["Basic", "Fundamentals"],
  "description": "Hovering is the absolute foundation of collective pitch helicopter flight. It involves maintaining the craft in a stationary, controlled position in the air, instantly reacting to wind and counter-torque to keep it perfectly still."
}
```

- **`id`**: A UUID v4 uniquely identifying this maneuver.
- **`vehicleType`**: Must be exactly `"Helicopter"`, `"Plane"`, or `"Drone"`.
- **`name`**: The display name of the maneuver (e.g., `"Hover"`, `"4-Point Hover"`).
- **`difficulty`**: An integer from `1` (Beginner) to `7` (Extreme/Advanced).
- **`tags`**: An array of string tag names. Tags must match names from `content/maneuver/tags.json`.
- **`description`**: A **full, detailed description** of the maneuver. This should explain what the maneuver is, why it matters, and how it is performed. Write it in general terms that apply to all variations. If generalizing is not possible (e.g., certain execution details differ significantly between variations), describe how the maneuver is performed for the default variation. This is the main body of knowledge for the maneuver — not just a brief intro.

---

## variation.json Schema

```json
{
  "id": "28302640-dc0e-4ac8-b214-a4c367288d07",
  "videoAssetName": "helicopter_hover_tail_in",
  "videoAssetId": "e0833c26-a107-4e14-b743-aa53dd4e2637",
  "name": "Tail-in Hover",
  "description": "### Execution Steps\n1. ...\n\n### Common Problems\n- **Problem:** ..."
}
```

- **`id`**: A UUID v4 uniquely identifying this variation.
- **`videoAssetName`**: The asset name used to resolve the demonstration video for this variation (must match an entry in the `asset.video` table).
- **`videoAssetId`**: The UUID of the corresponding video asset record.
- **`name`**: Short display name for this variation (e.g., `"Tail-in Hover"`, `"Nose-in Hover"`).
- **`description`**: A Markdown text block that **briefly explains how this variation differs from the base maneuver**. It should not repeat the full maneuver description. Focus on what is unique to this variation — different orientation, reversed controls, specific tips, etc. Follow the structure below.

---

## Variation Description Markdown Format

The `description` field inside `variation.json` is a Markdown text block. Adhere to the following structure:

### 1. Step-by-Step Guide
Concise numbered list of mechanical input steps specific to this variation.
*Use `### Execution Steps` as the header.*

### 2. Troubleshooting & Common Problems
Hints for common mistakes specific to this variation.
*Use `### Common Problems` as the header.*

---

### Example Variation Description

```markdown
### Execution Steps
1. Place the helicopter on a flat surface facing away from you (tail-in).
2. Smoothly increase collective until the helicopter lifts off to eye level.
3. Apply micro-corrections on the cyclic to stay over the spot.
4. Use rudder to keep the nose pointed directly away from you.
5. To land, slowly decrease collective until touchdown.

### Common Problems
- **Over-correcting (Pilot Induced Oscillations):** Keep inputs extremely small and wait for the helicopter to react before adding more.
- **Ground Effect Turbulence:** Hovering too low traps the helicopter in its own downwash. Climb through ground effect to a smoother hover altitude.
```
