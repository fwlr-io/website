import { ThemeContext } from "styled-components"
import { useState, useContext, useMemo } from "react"
import { useSpring, config as baseConfig } from "react-spring"
const config = { ...baseConfig.stiff, clamp: true }

// persist this state to local storage
// attempt to use it as a default next visit
export const useLocalStorageState = (key, defaultValue) => {
  const [state, setState] = useState(localStorage.getItem(key) || defaultValue)
  const setLocalStorageState = (value) => {
    localStorage.setItem(key, value)
    setState(value)
  }
  return [state, setLocalStorageState]
}

// handling application of dark/light mode and themes across the app with a single hook. Themes are automatically provided so you can do
// useAnimated({ color: 'red' })
// or
// useAnimated(theme => ({ color: theme.red }))
// This means it reads like "use animated (color... )" or "use animated (theme... )" which is cute.
// may be heavy, but optimise later if truly needed

export const useAnimated = (funcOrObj) => {
  const theme = useContext(ThemeContext)
  const memoConfig = useMemo(
    () => ({
      config,
      ...(typeof funcOrObj === "function" ? funcOrObj(theme) : funcOrObj),
    }),
    [funcOrObj, theme]
  )
  const anim = useSpring(memoConfig)
  return anim
}
