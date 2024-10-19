interface SpinnerProps {
  size?: number;
  color?: string;
  thickness?: number;
  speed?: number;
}

const Spinner: React.FC<SpinnerProps> = ({
  size = 40,
  color = '#007bff',
  thickness = 4,
  speed = 0.75
}) => {
  return (
    <div className="flex items-center justify-center">
      <div
        className="rounded-full animate-spin"
        style={{
          width: `${size}px`,
          height: `${size}px`,
          border: `${thickness}px solid #f3f3f3`,
          borderTop: `${thickness}px solid ${color}`,
          borderRadius: '50%',
          animation: `spin ${speed}s linear infinite`,
        }}
      />
    </div>
  );
};

export default Spinner