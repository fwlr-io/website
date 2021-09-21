import styled from "styled-components"

import { Col } from "Flex.js"

import MenuItem from "MenuItem.js"

const LeftPanelDiv = styled(Col)(({ theme }) => ({
  borderRight: `1px solid ${theme.fgDark}`,
  backgroundColor: theme.bgDefault,
  color: theme.fgDefault,
  flex: "1",
  alignItems: "stretch",
  gap: "2em",
}))

const TopMenuItem = styled(MenuItem)(({ theme }) => ({
  borderBottom: `1px solid ${theme.fgDark}`,
  color: theme.fgDefault,
  fontSize: "4em",
  fontWeight: "bold",
  padding: 12,
}))

const LeftPanel = ({ size }) => (
  <LeftPanelDiv size={size} className="code">
    <TopMenuItem>fwlr</TopMenuItem>
    <MenuItem>info</MenuItem>
    <MenuItem>work</MenuItem>
    <MenuItem>blog</MenuItem>
    <MenuItem>toys</MenuItem>
    <MenuItem>junk</MenuItem>
  </LeftPanelDiv>
)

export default LeftPanel
