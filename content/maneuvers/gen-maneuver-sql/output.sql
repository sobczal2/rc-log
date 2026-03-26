BEGIN;

TRUNCATE TABLE maneuver.maneuver_tag CASCADE;
TRUNCATE TABLE maneuver.tag CASCADE;
TRUNCATE TABLE maneuver.maneuver CASCADE;

INSERT INTO maneuver.tag (id, name) VALUES ('48446f12-70c8-4b17-bf4c-4219a83e1b45', 'High Energy');
INSERT INTO maneuver.tag (id, name) VALUES ('e87e2aa3-d79a-41a0-adb7-3631edecc2de', 'Collective Management');
INSERT INTO maneuver.tag (id, name) VALUES ('82bdac62-c4d2-49b7-b159-61dfcf3ad2da', 'Intermediate 3D');
INSERT INTO maneuver.tag (id, name) VALUES ('01b2260d-2964-48fc-b81e-a36ec7ea4223', 'Basic 3D');
INSERT INTO maneuver.tag (id, name) VALUES ('6203863b-6861-439d-8d5d-2f82b79b7b32', 'Orientation');
INSERT INTO maneuver.tag (id, name) VALUES ('790756b2-bcaa-43f1-ac5c-2986d0b6cb5f', 'Fundamentals');
INSERT INTO maneuver.tag (id, name) VALUES ('e224afed-92ab-4f37-acaa-5a3ec99fca22', 'Advanced');
INSERT INTO maneuver.tag (id, name) VALUES ('c7ab7452-28a9-4a69-832d-0bd565f70c2f', 'Basic');
INSERT INTO maneuver.tag (id, name) VALUES ('30635c53-5fe8-4605-be9c-b4eaeb4d70c0', 'Intermediate Sport');
INSERT INTO maneuver.tag (id, name) VALUES ('1e94f7e6-d25b-4351-9c96-1379a95b1ea5', 'Precision');
INSERT INTO maneuver.tag (id, name) VALUES ('39144e37-c70b-4b0d-bd5e-1437c88a84c0', 'Aerobatics');
INSERT INTO maneuver.tag (id, name) VALUES ('ed868903-74bc-45ae-8400-c1241d17d697', 'Advanced 3D');
INSERT INTO maneuver.tag (id, name) VALUES ('ed58e347-d6d9-4ede-8884-d240f87b03b0', 'Basic Sport');

INSERT INTO maneuver.maneuver (id, vehicle_type, name, description, difficulty, video_path) VALUES ('9052e630-bc26-4544-bf4c-3d3626cc9772', 'Helicopter', 'Funnel', 'A Funnel resembles a hurricane but is flown nose-down (or tail-down) in a much tighter, inverted cone shape. The helicopter slides rapidly sideways in a tight circular orbit with the rotor disc pitched nearly vertically inward.

### Execution Steps
1. Enter an upright hover, and tilt the helicopter roughly 60-80 degrees sharply onto its side (heavy left or right aileron).
2. Pull the nose aggressively toward the ground (forward elevator), pointing the canopy directly inward toward the center of your imaginary cone.
3. Inject positive collective instantly to prevent crashing—the main lift vector is now pushing horizontally inward to create the tight sideways circle, no longer pushing vertically upward against gravity.
4. Use the rudder to constantly lock the tail into the circle''s circumference as the helicopter rapidly orbits and slides sideways.
5. To exit, smoothly level the aileron while managing collective to arrest the heavy lateral slide momentum.

### Common Problems
- **Bailing Out Early:** Because the nose points dramatically downward while sliding sideways, the visual perspective often tricks nervous pilots into cutting the throttle. Commit to the collective to drive the maneuver.
- **Wobbly Orbits:** If the rudder isn''t locked perfectly with the sideways drift angle, the funnel will snake and look loose. Synchronize your aileron angle and rudder thoroughly.
- **Ground Strikes:** Without adequate positive collective to compensate for the extreme sideways pitch, the funnel''s tight radius will pull the helicopter directly into the ground like a dart.', 5, NULL);
INSERT INTO maneuver.maneuver_tag (maneuver_id, tag_id) VALUES ('9052e630-bc26-4544-bf4c-3d3626cc9772', '01b2260d-2964-48fc-b81e-a36ec7ea4223');
INSERT INTO maneuver.maneuver_tag (maneuver_id, tag_id) VALUES ('9052e630-bc26-4544-bf4c-3d3626cc9772', '6203863b-6861-439d-8d5d-2f82b79b7b32');

INSERT INTO maneuver.maneuver (id, vehicle_type, name, description, difficulty, video_path) VALUES ('04574ff5-19cf-479f-90e5-d633c2e95aa6', 'Helicopter', '4-Point Hover', 'The 4-Point Hover is a critical precision drill directly following basic hovers. The pilot rotates the helicopter through all four cardinal orientations (Tail-in, Side-in Right, Nose-in, Side-in Left), pausing for a few seconds perfectly still at each hard point. It is the absolute gatekeeper skill before attempting moving aerobatics.

### Execution Steps
1. Lift off and establish a stable, perfectly stationary tail-in hover.
2. Smoothly apply rudder to rotate the helicopter exactly 90 degrees to the right (viewing the left side of the canopy).
3. Arrest the rotation firmly. Hold this side-in hover for exactly 3 to 5 seconds, actively working the cyclic (pushing forward or backward in this perspective) to combat wind and stay locked in place.
4. Apply rudder to rotate another 90 degrees right until the nose is pointing completely at you (nose-in). Hold for 3 to 5 seconds. Remember: roll and pitch cyclic controls are completely visually reversed.
5. Rotate 90 degrees right to the final side-in orientation. Hold for 3 to 5 seconds.
6. Complete the square by returning perfectly to the tail-in starting orientation.

### Common Problems
- **Drifting During Rotation:** Beginners often introduce untended cyclic input when shifting the rudder. Concentrate heavily on isolating your thumb movements to solely pivot the tail unhindered.
- **Panicking Nose-In:** If the helicopter drifts while nose-in, the reversed cyclic controls usually trigger panic. Remember the golden rule: ''Push the cyclic stick toward the mistake'' to seamlessly correct drift while nose-in.
- **Impatient Pauses:** Not holding the hover long enough at each cardinal point defeats the purpose. Mentally count out loud to five before clicking the next rotation to ensure total, absolute stick control has been demonstrated.', 3, NULL);
INSERT INTO maneuver.maneuver_tag (maneuver_id, tag_id) VALUES ('04574ff5-19cf-479f-90e5-d633c2e95aa6', '30635c53-5fe8-4605-be9c-b4eaeb4d70c0');
INSERT INTO maneuver.maneuver_tag (maneuver_id, tag_id) VALUES ('04574ff5-19cf-479f-90e5-d633c2e95aa6', '1e94f7e6-d25b-4351-9c96-1379a95b1ea5');

INSERT INTO maneuver.maneuver (id, vehicle_type, name, description, difficulty, video_path) VALUES ('7e3c29c0-7a13-4e23-8010-77a14ca24e77', 'Helicopter', 'Hurricane', 'A Hurricane is an aggressive, high-energy 3D maneuver characterized by flying very fast, large, sweeping circles perfectly backwards. It demands total mastery of fast backward flight and heavy collective management to maintain immense speed and centrifugal locking force.

### Execution Steps
1. From an upright hover, push the tail directly up into forward flight, then sharply pivot 180 degrees to begin flying at high speed tail-first.
2. Apply continuous side-cyclic (aileron) in the direction of the desired circle, tilting the rotor disc heavily inward toward the center.
3. Maintain constant forward elevator (which pushes the tail aggressively backwards in this orientation) to keep the speed extremely high.
4. Inject large amounts of collective pitch to combat the extreme bank angle of the rotor disc and prevent the helicopter from losing altitude throughout the sweep.
5. Adjust rudder constantly to keep the tail leading flawlessly along the outer flight path circumference.

### Common Problems
- **Loss of Speed:** If the elevator isn''t held forward enough, the tail won''t drive through the circle. Keeping speed high generates the required centrifugal "lock" for a clean hurricane.
- **Altitude Bleed:** A steep inward bank angle rapidly dumps vertical lift. You must aggressively pull collective to keep the helicopter floating while banked.
- **Velocity Disorientation:** Given the speed and backward orientation, misinterpreting the tail''s track quickly leads to catastrophic dirt naps. Practice slow backwards circuits extensively before attempting hurricane speeds.', 6, NULL);
INSERT INTO maneuver.maneuver_tag (maneuver_id, tag_id) VALUES ('7e3c29c0-7a13-4e23-8010-77a14ca24e77', 'ed868903-74bc-45ae-8400-c1241d17d697');
INSERT INTO maneuver.maneuver_tag (maneuver_id, tag_id) VALUES ('7e3c29c0-7a13-4e23-8010-77a14ca24e77', '48446f12-70c8-4b17-bf4c-4219a83e1b45');

INSERT INTO maneuver.maneuver (id, vehicle_type, name, description, difficulty, video_path) VALUES ('ef420b11-c23c-498b-8471-b6274e34b03d', 'Helicopter', 'Tic-Toc', 'The Tic-Toc is aggressive, rhythmic 3D flying where the helicopter swings rapidly back and forth between two extreme pitch angles (typically 12-o''clock and 6-o''clock) without moving forward or backward, mimicking the steady pendulum swing of a grandfather clock.

### Execution Steps
1. Pop the helicopter up pointing perfectly vertical (nose straight up to the sky inside a stationary hover).
2. Apply strong positive collective while briefly pushing the elevator forward, brutally launching the helicopter backward.
3. Immediately flip the elevator backward and simultaneously pull total negative collective, reversing the momentum and throwing the helicopter forcefully forward.
4. Rhythmically bounce between maximum positive collective and maximum negative collective at the extreme ends of the pendulum swing.
5. Balance elevator timing intimately so the helicopter remains completely suspended in place over the same patch of grass while snapping between pitches.

### Common Problems
- **Losing Altitude (''Sinking Tic-Tocs''):** If the collective isn''t applied fast or aggressively enough at the apex of each extreme swing, the helicopter bleeds altitude with every stroke.
- **Traveling:** If you spend slightly more time on positive collective than negative collective, the helicopter will ''walk'' forward or backward. The strokes must be perfectly symmetrical.
- **Bogging the Motor:** Mashing complete collective pitch before the elevator finishes pitching heavily drags down the head-speed. ''Lead'' the movement softly with the cyclic, then punch the collective pitch to finish the stroke.', 6, NULL);
INSERT INTO maneuver.maneuver_tag (maneuver_id, tag_id) VALUES ('ef420b11-c23c-498b-8471-b6274e34b03d', '82bdac62-c4d2-49b7-b159-61dfcf3ad2da');
INSERT INTO maneuver.maneuver_tag (maneuver_id, tag_id) VALUES ('ef420b11-c23c-498b-8471-b6274e34b03d', 'e87e2aa3-d79a-41a0-adb7-3631edecc2de');

INSERT INTO maneuver.maneuver (id, vehicle_type, name, description, difficulty, video_path) VALUES ('c20c268f-555a-4dbe-8aa0-683fa308a529', 'Helicopter', 'Stall Turn', 'A Stall Turn (often called a Hammerhead) is a graceful end-of-pass maneuver where the helicopter flies straight up, stops completely, pivots 180 degrees on its tail, and dives straight back down perfectly parallel to its entry path.

### Execution Steps
1. Enter fast forward flight precisely horizontal to the ground.
2. Pull back smoothly on the elevator to pitch the helicopter 90 degrees straight up into a vertical climb.
3. As the helicopter loses upward momentum and nears a complete stop (the apex), apply full rudder to kick the tail 180 degrees around.
4. As the nose points straight down, center the rudder and let the helicopter briefly dive to regain airspeed.
5. Gently pull back on the elevator to exit the dive seamlessly back into horizontal forward flight in the opposite direction.

### Common Problems
- **Pivoting Too Early:** If you apply rudder while still traveling upwards quickly, the helicopter will perform a messy, climbing pirouette instead of pivoting cleanly on its axis. Wait for the exact moment of zero vertical speed.
- **Flopping Backwards:** If you wait too long after stopping, the helicopter will begin to fall straight backwards before you pivot. Timing the rudder application is crucial.
- **Losing the Vertical Line:** Crosswinds or uneven cyclic input during the climb can cause the helicopter to lean sideways or backward. Make constant micro-adjustments during the climb to maintain a perfect 90-degree pitch.', 2, helicopter_stall_turn.mp4);
INSERT INTO maneuver.maneuver_tag (maneuver_id, tag_id) VALUES ('c20c268f-555a-4dbe-8aa0-683fa308a529', 'ed58e347-d6d9-4ede-8884-d240f87b03b0');
INSERT INTO maneuver.maneuver_tag (maneuver_id, tag_id) VALUES ('c20c268f-555a-4dbe-8aa0-683fa308a529', '39144e37-c70b-4b0d-bd5e-1437c88a84c0');

INSERT INTO maneuver.maneuver (id, vehicle_type, name, description, difficulty, video_path) VALUES ('e68e18db-a8b1-44ab-9d60-405e5bd372cd', 'Helicopter', 'Hover', 'Hovering is the absolute foundation of collective pitch helicopter flight. It involves maintaining the craft in a stationary, controlled position in the air, instantly reacting to wind and counter-torque to keep it perfectly still.

### Execution Steps
1. Ensure the helicopter is placed on a flat, clear surface facing away from you (tail-in).
2. Smoothly increase the collective pitch/throttle until the helicopter becomes light on its skids.
3. Add a touch more collective to gently break ground effect and lift off to about eye level (roughly 4-6 feet high).
4. Constantly input micro-corrections on the cyclic (elevator and aileron) to stay over the spot.
5. Use the rudder (tail rotor) to keep the nose pointed directly away from you.
6. To land, slowly decrease collective until the skids touch down, then lower fully to idle.

### Common Problems
- **Over-correcting (Pilot Induced Oscillations):** Beginners often make control inputs that are too large, leading to swinging. Keep your thumb/finger movements extremely small (micromillimeters) and wait a split second for the helicopter to react.
- **Losing Orientation:** If the nose starts drifting to the side, don''t panic. Gently apply rudder to bring the tail back to pointing straight at you. Focus on the nose of the canopy.
- **Ground Effect Turbulence:** Hovering too low (under 2 feet) traps the helicopter in its own chaotic downwash. Punch completely through the ground effect to reach a higher, smoother hover.', 1, helicopter_hover.mp4);
INSERT INTO maneuver.maneuver_tag (maneuver_id, tag_id) VALUES ('e68e18db-a8b1-44ab-9d60-405e5bd372cd', 'c7ab7452-28a9-4a69-832d-0bd565f70c2f');
INSERT INTO maneuver.maneuver_tag (maneuver_id, tag_id) VALUES ('e68e18db-a8b1-44ab-9d60-405e5bd372cd', '790756b2-bcaa-43f1-ac5c-2986d0b6cb5f');

INSERT INTO maneuver.maneuver (id, vehicle_type, name, description, difficulty, video_path) VALUES ('d3be8246-4904-4531-8ebf-db995d558602', 'Helicopter', 'Pirouetting Hover', 'A Pirouetting Hover requires the pilot to maintain a stationary hover over a single spot while the helicopter constantly spins (pirouettes) around its main vertical axis. This maneuver tests a pilot''s ability to seamlessly master all four orientation profiles (tail-in, nose-in, side-in left, side-in right).

### Execution Steps
1. Establish a stable tail-in hover at a comfortable altitude (typically eye-level or slightly higher).
2. Apply a small, constant amount of rudder input to begin a slow pirouette. Commit to the spin direction (left or right).
3. As the nose rotates, your cyclic (elevator and aileron) inputs must continuously cycle to match the orientation. For example, when the nose is pointing right, forward elevator actually pushes the heli to the right.
4. Stir the cyclic stick in a circular motion that exactly matches the rotation speed of the tail to seamlessly counteract any drift.
5. Modulate your collective continuously to maintain a completely flat altitude during the rotation.

### Common Problems
- **Drifting Away From the Spot:** Usually caused by your thumb trailing or leading the "stirring" motion relative to the actual nose orientation. Slow down your pirouette rate to give your brain time to compute the required cyclic inputs.
- **Altitude Pumping:** As the tail rotor constantly sweeps, it demands varying amounts of power from the main rotor system, causing the heli to bob up and down. Focus on smooth collective management independently of your cyclic hand.
- **Panic at Nose-in:** If orientation is lost while the nose is pointing towards you, immediately halt the rudder and bail out using a combined pitch-out climb, or quickly snap the tail back towards you.', 4, helicopter_pirouetting_hover.mp4);
INSERT INTO maneuver.maneuver_tag (maneuver_id, tag_id) VALUES ('d3be8246-4904-4531-8ebf-db995d558602', 'e224afed-92ab-4f37-acaa-5a3ec99fca22');
INSERT INTO maneuver.maneuver_tag (maneuver_id, tag_id) VALUES ('d3be8246-4904-4531-8ebf-db995d558602', '6203863b-6861-439d-8d5d-2f82b79b7b32');

COMMIT;
