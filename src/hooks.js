import { ThemeContext } from "styled-components"
import { useSpring, config as baseConfig } from "react-spring"
import { useState, useContext, useMemo } from "react"

export const useLocalStorageState = (key, defaultValue) => {
  const [state, setState] = useState(localStorage.getItem(key) || defaultValue)
  const setLocalStorageState = (value) => {
    localStorage.setItem(key, value)
    setState(value)
  }
  return [state, setLocalStorageState]
}

// handling dark mode / light mode and themes across the app with a single hook

export const config = { ...baseConfig.stiff, clamp: true }

export const useTheme = () => {
  const theme = useContext(ThemeContext)
  return theme
}

export const useThemeSpring = (func) => {
  const theme = useContext(ThemeContext)
  const memoConfig = useMemo(
    () => ({
      config,
      ...func(theme),
    }),
    [func, theme]
  )
  const spring = useSpring(memoConfig)
  return spring
}

// streamline using animations
// export const useAnimated = (funcOrObj) => {
//   const memoConfig = useMemo(
//     () => ({
//       config,
//       ...func()
//     })
//     )
// }
