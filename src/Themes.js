import { createContext, useMemo, useCallback } from "react"
import { ThemeProvider } from "styled-components"
import themeDefs from "ThemeDefs.js"
// const themeList = Object.keys(themeDefs)
import { useLocalStorageState } from "hooks.js"

// chosenMode ("dark", "light")
// toggleMode (func that swaps between "light" and "dark")
// chosenTheme ("atelierDune", "solarized", ...)
// chooseTheme (func that returns a valid theme)

export const ThemeStateContext = createContext({
  chosenMode: "dark", // "dark", "light"
  toggleMode: () => {}, // swaps between "dark" and "light"
  chosenTheme: "atelierDune",
  chooseTheme: () => {}, // takes a name, returns a theme object
})

const ThemeManager = ({ children }) => {
  // prepare this context's state
  const [chosenTheme, chooseTheme] = useLocalStorageState(
    "themeName",
    "atelierDune"
  )
  const [chosenMode, setMode] = useLocalStorageState("themeMode", "dark")
  const toggleMode = useCallback(
    () => setMode(chosenMode === "dark" ? "light" : "dark"),
    [setMode, chosenMode]
  )

  // set up the theme state manager that we use to switch modes/themes
  const themeState = useMemo(
    () => ({
      chosenMode,
      toggleMode,
      chosenTheme,
      chooseTheme,
    }),
    [chosenMode, toggleMode, chosenTheme, chooseTheme]
  )

  // set up the actual theme manager from styled
  // objects are all pre-defined so there's no need to memo
  const theme = themeDefs[chosenTheme][chosenMode]

  return (
    <ThemeStateContext.Provider value={themeState}>
      <ThemeProvider theme={theme}>{children}</ThemeProvider>
    </ThemeStateContext.Provider>
  )
}

export default ThemeManager
