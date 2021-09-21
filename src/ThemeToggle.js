import styled from "styled-components"

const DullThemeToggle = ({ className, themeState, setTheme }) => {
  const handleClick = () => {
    setTheme(themeState === "dark" ? "light" : "dark")
  }
  return (
    <div className={className + " code"} onClick={handleClick}>
      {themeState === "dark" ? "L" : "D"}
    </div>
  )
}

const ThemeToggle = styled(DullThemeToggle)(({ theme }) => ({
  color: theme.fgDefault,
  position: "absolute",
  right: 10,
  top: 10,
  lineHeight: 1,
  fontSize: "2em",
  fontWeight: "bold",
}))

export default ThemeToggle
