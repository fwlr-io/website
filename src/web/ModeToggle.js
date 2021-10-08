import { useContext, useCallback } from "react"
import { ThemeStateContext } from "Themes.js"
import styled from "styled-components"
import { animated } from "react-spring"
import { useAnimated } from "hooks.js"
import { cyan } from "ThemeDefs.js"

const ToggleContainer = styled(animated.div)`
  display: flex;
  flex-direction: row;
  gap: 1rem;
  position: absolute;
  right: 10px;
  top: 10px;
  line-height: 1;
  font-size: 2rem;
  font-weight: bold;
  cursor: pointer;
`
const animThemeToggle = (theme) => ({
  color: theme.darkForeground,
})

const StyledToggle = styled.div`
  color: inherit;
  &:hover {
    color: ${cyan};
  }
`

const ThemeToggle = () => {
  const { chosenMode, toggleMode, chosenTheme, chooseTheme } =
    useContext(ThemeStateContext)
  const toggleTheme = useCallback(() => {
    chooseTheme(chosenTheme === "atelierDune" ? "solarized" : "atelierDune")
  }, [chosenTheme, chooseTheme])
  const anim = useAnimated(animThemeToggle)
  return (
    <ToggleContainer className="code" style={anim}>
      <StyledToggle onClick={toggleMode}>
        {chosenMode === "dark" ? "L" : "D"}
      </StyledToggle>
      <StyledToggle onClick={toggleTheme}>T</StyledToggle>
    </ToggleContainer>
  )
}

export default ThemeToggle
