import MessageElm from "@/components/MessageElm";
import Grid from "@mui/material/Grid/Grid";
import { Message } from "@/structs";
import { get_avatar } from "@/avatar";

interface OdaiElmProps {
  odai: Message;
  melchiorAnswer: Message | undefined;
  balthasarAnswer: Message | undefined;
  casperAnswer: Message | undefined;
}

function OdaiElm(props: OdaiElmProps) {
  const { odai, melchiorAnswer, balthasarAnswer, casperAnswer } = props;

  const userElm = <MessageElm text={odai.content} avatar={get_avatar(odai)} />;

  const melchiorElm = melchiorAnswer ? (
    <MessageElm
      text={melchiorAnswer.content}
      avatar={get_avatar(melchiorAnswer)}
    />
  ) : (
    <></>
  );

  const balthasarElm = balthasarAnswer ? (
    <MessageElm
      text={balthasarAnswer.content}
      avatar={get_avatar(balthasarAnswer)}
    />
  ) : (
    <></>
  );

  const casperElm = casperAnswer ? (
    <MessageElm text={casperAnswer.content} avatar={get_avatar(casperAnswer)} />
  ) : (
    <></>
  );

  return (
    <Grid
      container
      sx={{
        width: "100%",
        alignItems: "center",
        justifyContent: "center",
        m: "0 auto",
      }}
    >
      {[userElm, melchiorElm, balthasarElm, casperElm].map((elm, id) => (
        <Grid
          item
          xs={12}
          sx={{
            alignItems: "center",
            justifyContent: "center",
          }}
          key={id}
        >
          {elm}
        </Grid>
      ))}
    </Grid>
  );
}

export default OdaiElm;
