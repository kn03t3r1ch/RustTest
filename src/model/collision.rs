/**
 * Rotates coordinate system for velocities
 *
 * Takes velocities and alters them as if the coordinate system they're on was rotated
 *
 * @param  Object | velocity | The velocity of an individual particle
 * @param  Float  | angle    | The angle of collision between two objects in radians
 * @return Object | The altered x and y velocities after the coordinate system has been rotated
 */

fn rotate(velocity: f32, angle: f32) {
    let rotated_velocities = (
        velocity.x * Math.cos(angle) - velocity.y * Math.sin(angle),
        velocity.x * Math.sin(angle) + velocity.y * Math.cos(angle)
    );

    return rotated_velocities;
}

/**
 * Swaps out two colliding particles' x and y velocities after running through
 * an elastic collision reaction equation
 *
 * @param  Object | particle      | A particle object with x and y coordinates, plus velocity
 * @param  Object | otherParticle | A particle object with x and y coordinates, plus velocity
 * @return Null | Does not return a value
 */

fn resolveCollision(particle: ball, otherParticle) {
    let xVelocityDiff = particle.velocity.x - otherParticle.velocity.x;
    let yVelocityDiff = particle.velocity.y - otherParticle.velocity.y;

    let xDist = otherParticle.x - particle.x;
    let yDist = otherParticle.y - particle.y;

    // Prevent accidental overlap of particles
    if (xVelocityDiff * xDist + yVelocityDiff * yDist >= 0) {

        // Grab angle between the two colliding particles
        let angle = -Math.atan2(otherParticle.y - particle.y, otherParticle.x - particle.x);

        // Store mass in var for better readability in collision equation
        let m1 = particle.mass;
        let m2 = otherParticle.mass;

        // Velocity before equation
        let u1 = rotate(particle.velocity, angle);
        let u2 = rotate(otherParticle.velocity, angle);

        // Velocity after 1d collision equation
        let v1 = { x: u1.x * (m1 - m2) / (m1 + m2) + u2.x * 2 * m2 / (m1 + m2), y: u1.y };
        let v2 = { x: u2.x * (m1 - m2) / (m1 + m2) + u1.x * 2 * m2 / (m1 + m2), y: u2.y };

        // Final velocity after rotating axis back to original location
        let vFinal1 = rotate(v1, -angle);
        let vFinal2 = rotate(v2, -angle);

        // Swap particle velocities for realistic bounce effect
        particle.velocity.x = vFinal1.x;
        particle.velocity.y = vFinal1.y;

        otherParticle.velocity.x = vFinal2.x;
        otherParticle.velocity.y = vFinal2.y;
    }
}