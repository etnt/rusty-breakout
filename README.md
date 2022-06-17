# rusty-breakout: experiments with the rusty-engine

After having finished `rusty-asteroids`, I wanted
to try and see if I could write a simple `Breakout` game.

## Install and run

After cloning, download the assets:

    cd rusty-breakout
    curl -L https://github.com/CleanCut/rusty_engine/archive/refs/heads/main.tar.gz | tar -zxv --strip-components=1 rusty_engine-main/assets

then compile and run:

    cargo run --release
    
## Controls and how to play

Balls are fired with the `SPACE` key, movement is done by 
the `LEFT` and `RIGHT` key. Slanting the padel is done
by the `A` (left up slant) key and the `D` (right up slant) key.

You win when all the bricks are removed, and you loose when
all the balls are gone and there is no more ammunition left.

Hitting a red brick will add one ball to the ammunition store.


## Some notes

First I started to think about how to calculate the bounces off
a wall when a ball hits it. I scratched my head and scribbled
down some situations. I came up with three cases:

![](breakout-angle-calcs.png)


1. The ball hits the wall perpendicular. In this case it should
bounce straight back, i.e we rotate the incoming angle with PI radians.

2. The ball hits the wall with a shot rotation less than PI.
The answer turned out to be `2*W-S` (W=rotation of wall, S=rotation of shot).

2. The ball hits the wall with a shot rotation greater than PI.
The answer turned out to be `2*(PI+W)-S`.


To create the walls I made use of the nifty `level_creator` tool.
I started by constructing the walls by smaller "brick" pieces, but
I soon realized that it was easer to just use one brick and then
scale it up really large and position it mostly off screen. This
way I got nice thin solid walls.

I continued with the `level_creator` to form an upside down pyramid of
blocks that I adjusted a bit by hand. Then a little loop to create a row
of such pyramid blocks.

Since the Player only moves horizontally I wanted to have the balls
to be fired in various directions. A random angle range centered
around the Y-axis (ᴨ) solved that:

``` rust
    sprite.rotation = thread_rng().gen_range(FRAC_1_PI..FRAC_PI_2);
```

The collision handling was very similar to `rusty-asteroids` and with that
I had a fully working `rusty-breakout` game... :-)

![](pyramids.png)

Then I started to think about ball spin and how that could affect the
bounce calculation but I couldn't come up with a good solution.
Instead I implemented a way to slant the Player "padel". By pressing
either `A` or `D` the padel will now rotate left/right a small fraction
to give the padel a slant which affects the calculation of the
outgoing bounce. I was quite happy with how it turned out.

I added four rows of pyramid bricks and to make it even more fun
I randomly generated approximately 10% red bricks, where hitting
a red brick will add to the ammunition store.

This concluded the rules of the game. You win when all the bricks
are removed, and you loose when all the balls are gone and there is
no more ammunition left. I was surprised how quickly I managed
to create this game. The tricky part was of course the bounce
calculations; for the other parts I had good use of the techniques
I had gathered from the `rusty-asteroids` game.
