import styled from "styled-components"
import { animated } from "react-spring"
import { useAnimated } from "hooks.js"
import { green } from "ThemeDefs.js"
import ModeToggle from "./ModeToggle.js"
import { NavLink } from "react-router-dom"

const NavBarDiv = styled(animated.div)`
  font-size: 2rem;
  height: 3rem;
  flex: none;
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-around;
`
const animNavBarDiv = (theme) => ({
  backgroundColor: theme.darkBackground,
  color: theme.darkForeground,
})

const NavItem = styled(NavLink)`
  flex: 3;
  text-align: center;
  text-decoration: none;
  color: inherit;
  &.active {
    color: ${green};
  }
`

const NavBar = () => {
  const animNav = useAnimated(animNavBarDiv)
  return (
    <NavBarDiv className="code" style={animNav}>
      <NavItem to="/" end>
        info
      </NavItem>
      <NavItem to="blog">blog</NavItem>
      <ModeToggle />
    </NavBarDiv>
  )
}

export default NavBar
