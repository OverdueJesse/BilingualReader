interface Props {
  blob: Blob;
}

const Thumbnail = ({ blob }: Props) => {
  return (
    <img
      src={window.URL.createObjectURL(blob)}
      style={{ width: "300px", height: "500px", objectFit: "cover" }}
    />
  );
};

export default Thumbnail;
