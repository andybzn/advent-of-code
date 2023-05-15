import { readInput } from './helpers/readFile.mjs'
import { handleAnswer } from './helpers/parseAnswer.mjs'

function partOne (input) {
  let clock = process.hrtime.bigint()
  let floorNumber = 0

  input.forEach(floor => {
    switch (floor) {
      case '(':
        floorNumber += 1
        return
      case ')':
        floorNumber -= 1
    }
  })

  clock = ((process.hrtime.bigint()) - clock)
  const result = {
    answer: floorNumber,
    time: clock
  }

  return result
}

function partTwo (input) {
  let clock = process.hrtime.bigint()
  let floorNumber = 0
  let basement = false
  let basementPosition = 1

  input.forEach(floor => {
    switch (floor) {
      case '(':
        floorNumber += 1
        break
      case ')':
        floorNumber -= 1
        break
    }
    if (!basement) {
      basementPosition += 1
      if (floorNumber === 0) {
        basement = true
      } else {
        basement = false
      }
    }
  })

  clock = ((process.hrtime.bigint()) - clock)
  const result = {
    answer: basementPosition,
    time: clock
  }

  return result
}

const handleInput = (input) => { return { partOne: partOne(input), partTwo: partTwo(input) } }

// let's go
readInput('../../inputs/day_1.txt')
  .then(res => handleInput([...res]))
  .then(answer => handleAnswer(answer))
  .then(result => console.table(result))
  .catch(error => console.error(error))
