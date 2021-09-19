import styled from "styled-components"

import { Row, Item } from "Flex.js"
import LeftPanel from "LeftPanel.js"

const Main = styled(Row)({
  width: "100%",
  height: "100%",
})

const Content = styled(Item)({
  backgroundColor: "white",
})

function App() {
  return (
    <Main>
      <LeftPanel size={1}></LeftPanel>
      <Content size={5}></Content>
    </Main>
  )
}

export default App
