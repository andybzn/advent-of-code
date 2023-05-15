import fs from 'fs'

export async function readInput (filepath) {
  console.info('readInput from: ' + filepath)

  try {
    const fileData = fs.readFileSync(filepath, 'utf-8')
    return fileData
  } catch (err) {
    throw new Error(err)
  }
}
