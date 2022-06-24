package main

import (
	"fmt"
	"math"
	//"syscall/js"
)

type human struct {
	name      string
	greetings string
}

func (h *human) setGreetings(txt string) {
	h.greetings = "😃 Hello " + txt
}

type Constraints struct {
	border      float64
	width       float64
	height      float64
	maxVelocity float64
}

type Cow struct {
	nickName string
	x float64
	y float64
	constraints Constraints
	xVelocity float64
	yVelocity float64
}

func (cow *Cow) initialize(nickName string, x float64, y float64, constraints Constraints) {
	cow.nickName = nickName
	cow.x = x
	cow.y = y
	cow.constraints = constraints
	cow.xVelocity = 1.0
	cow.yVelocity = -1.0
}

func (this *Cow) move() {
	this.x += this.xVelocity
	this.y += this.yVelocity
	if(this.x <= this.constraints.border || this.x >= this.constraints.width - this.constraints.border) {
		this.x -= this.xVelocity
		this.x = math.Max(this.x, this.constraints.border)
		this.x = math.Min(this.x, this.constraints.width - this.constraints.border)
		this.xVelocity = -this.xVelocity
		this.x += this.xVelocity
	}
	if(this.y <= this.constraints.border || this.y >= this.constraints.height - this.constraints.border) {
		this.y -= this.yVelocity
		this.y = math.Max(this.y, this.constraints.border)
		this.y = math.Min(this.y, this.constraints.height - this.constraints.border)
		this.yVelocity = -this.yVelocity
		this.y += this.yVelocity
	}   

}

func (this *Cow) distance(boid Cow) float64 {
	var distX = this.x - boid.x
	var distY = this.y - boid.y
	return math.Sqrt(distX * distX + distY * distY)
}

func (this *Cow) moveAway (boids []Cow, minDistance float64) {
	var distanceX = 0.0
	var distanceY = 0.0
	var numClose = 0.0

	//     for i := 1; i <= 5; i++ {
		

	for i := 0; i < len(boids); i++ {
		var boid = boids[i];

		if(boid.x == this.x && boid.y == this.y) {
			fmt.Println("continue")
			continue
		} 

		var distance = this.distance(boid)

		fmt.Println("distance:", distance, "minDistance", minDistance)

		if(distance < minDistance) {
			numClose++
			var xdiff = (this.x - boid.x)
			var ydiff = (this.y - boid.y)

			if(xdiff >= 0) {
				xdiff = math.Sqrt(minDistance) - xdiff
			} else if(xdiff < 0) {
				xdiff = -math.Sqrt(minDistance) - xdiff
			}

			if(ydiff >= 0) {
				ydiff = math.Sqrt(minDistance) - ydiff
			} else if(ydiff < 0) {
				ydiff = -math.Sqrt(minDistance) - ydiff
			}
			distanceX += xdiff
			distanceY += ydiff
			//boid = nil;
		}
	}

	if(numClose == 0) {
		fmt.Println("nothing")
	} else {
		fmt.Println("something")
		this.xVelocity -= distanceX / 5
		this.yVelocity -= distanceY / 5
	}


}



func main() {

	constraints := Constraints{
		border:      5.0,
		width:       800.0,
		height:      800.0,
		maxVelocity: 5.0,
	}

	/*
	bob := Cow{
		nickName: "Bob",
		x: 10.0,
		y: 10.0,
		constraints: constraints,
		xVelocity: 1.0,
		yVelocity: -1.0,
	}
	*/
	bob := Cow{}
	bob.initialize("Bob", 10.0, 10.0, constraints)

	sam := Cow{}
	sam.initialize("Sam", 10.0, 10.0, constraints)
	sam.move()
	sam.move()

	fmt.Println(bob.nickName, bob.constraints.maxVelocity)

	fmt.Println(bob.x, bob.y)
	bob.move()
	fmt.Println(bob.x, bob.y)
	fmt.Println(bob.distance(sam))

	h := human{name: "Bob", greetings: ""}
	h.setGreetings("I'm Bob")
	fmt.Println(h.greetings)

	fmt.Println(sam.x, sam.y, sam.xVelocity, sam.yVelocity)
	sam.moveAway([]Cow{sam, bob}, 3.0)
	fmt.Println(sam.x, sam.y, sam.xVelocity, sam.yVelocity)


	<-make(chan bool)

	

}
