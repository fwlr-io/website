import { Children } from "react"
import styled from "styled-components"
import { cyan } from "ThemeDefs.js"

const StyledParagraph = styled.div`
  display: flex;
  flex-flow: row wrap;
  justify-content: space-around;
`
const ParagraphContainer = styled.div`
  width: 60ch;
  flex: none;
  display: flex;
  justify-content: start;
`

export const WebParagraph = ({ children }) => {
  return (
    <StyledParagraph>
      {Children.map(children || null, (child, i) => (
        <ParagraphContainer>{child}</ParagraphContainer>
      ))}
    </StyledParagraph>
  )
}

export const WebSerif = styled.p.attrs({ className: "serif" })`
  text-align: justify;
  margin-block-start: 0;
  margin-block-end: 0;
`

export const WebCode = styled.code`
  line-height: 1.4;
`

export const WebImage = styled.div`
  width: 300px;
  height: 400px;
  background-color: ${cyan};
  margin-left: auto;
  margin-right: auto;
`
