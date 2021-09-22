import styled from "styled-components"

const StyledThemeToggle = styled.div`
  color: ${(props) => props.theme.bgSuperlight};
  position: absolute;
  right: 10px;
  top: 10px;
  line-height: 1;
  font-size: 2em;
  font-weight: bold;
  cursor: pointer;
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
//
// const DullThemeToggle = ({ className, themeState, setTheme }) => {
//   const handleClick = () => {
//     setTheme(themeState === "dark" ? "light" : "dark")
//   }
//   return (
//     <div className={className + " code"} onClick={handleClick}>
//       {themeState === "dark" ? "L" : "D"}
//     </div>
//   )
// }
//
//
//
// const ThemeToggle = styled(DullThemeToggle)(({ theme }) => ({
//   color: theme.fgDefault,
//   position: "absolute",
//   right: 10,
//   top: 10,
//   lineHeight: 1,
//   fontSize: "2em",
//   fontWeight: "bold",
// }))

export default ThemeToggle
