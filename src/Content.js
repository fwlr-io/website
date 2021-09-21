import styled from "styled-components"
import { Item } from "Flex.js"

const Content = styled(Item)(({ theme }) => ({
  backgroundColor: theme.bgDark,
}))

export default Content
