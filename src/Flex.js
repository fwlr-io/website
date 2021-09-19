import styled from "styled-components"

export const Row = styled.div({
  display: "flex",
  flexDirection: "row",
})

export const Col = styled.div({
  display: "flex",
  flexDirection: "column",
})

export const Item = styled.div(({ size }) => ({
  flex: size,
}))

// import { Grid, Row, Col, Item } from "Flex.js"
