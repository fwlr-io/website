import { useState } from "react"

export const useLocalStorageState = (key, defaultValue) => {
  const [state, setState] = useState(localStorage.getItem(key) || defaultValue)
  const setLocalStorageState = (value) => {
    localStorage.setItem(key, value)
    setState(value)
  }
  return [state, setLocalStorageState]
}
