import styled from "styled-components"

const StyledThemeToggle = styled.div`
  position: absolute;
  right: 10px;
  top: 10px;
  line-height: 1;
  font-size: 2em;
  font-weight: bold;
  cursor: pointer;
  color: ${(props) => props.theme.bgSuperlight};
  &:hover {
    color: ${(props) => props.theme.teal};
  }
`

const ThemeToggle = ({ themeState, setTheme }) => {
  const isDark = themeState === "dark"
  const handleClick = () => {
    setTheme(isDark ? "light" : "dark")
  }
  return (
    <StyledThemeToggle className="code" onClick={handleClick}>
      {isDark ? "L" : "D"}
    </StyledThemeToggle>
  )
}

export default ThemeToggle
