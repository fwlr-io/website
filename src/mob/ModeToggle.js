import { useContext, useCallback } from "react"
import { ThemeStateContext } from "Themes.js"
import styled from "styled-components"
import { cyan } from "ThemeDefs.js"

const StyledThemeToggle = styled.div`
  color: ${cyan};
`

const ThemeToggle = () => {
  const { chosenMode, toggleMode, chosenTheme, chooseTheme } =
    useContext(ThemeStateContext)
  const toggleTheme = useCallback(() => {
    chooseTheme(chosenTheme === "atelierDune" ? "solarized" : "atelierDune")
  }, [chosenTheme, chooseTheme])

  return (
    <>
      <StyledThemeToggle className="code" onClick={toggleMode}>
        {chosenMode === "dark" ? "L" : "D"}
      </StyledThemeToggle>
      <StyledThemeToggle className="code" onClick={toggleTheme}>
        T
      </StyledThemeToggle>
    </>
  )
}

export default ThemeToggle
