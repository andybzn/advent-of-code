export function handleAnswer (answer) {
  const table = {
    partOne: {
      answer: answer.partOne.answer,
      time: parseFloat((Number(answer.partOne.time) / 1000000).toFixed(2))
    },
    partTwo: {
      answer: answer.partTwo.answer,
      time: parseFloat((Number(answer.partTwo.time) / 1000000).toFixed(2))
    }
  }
  return table
}
