import styled from "styled-components"

const StyledInfo = styled.ul`
  max-width: 700px;
  min-width: 400px;
  font-size: 2em;
  margin: auto;
`

const Info = () => {
  return (
    <StyledInfo>
      <li>Scott Fowler</li>
      <li>React and Javascript</li>
      <li>scott@fwlr.io</li>
      <li>github.com/fwlr.io</li>
    </StyledInfo>
  )
}

export default Info
